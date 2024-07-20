use bevy::{app::PluginGroupBuilder, asset::AssetMetaCheck, input::keyboard::KeyboardInput, prelude::*, window::{Cursor, WindowFocused, WindowResolution}};
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

use asset_loader::AssetLoaderPlugin;
use bevy_rapier3d::{plugin::{NoUserData, RapierPhysicsPlugin}, render::RapierDebugRenderPlugin};

use camera::MainCameraPlugin;
use inputs::InputsPlugin;
use states::GameState;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use world::WorldPlugin;

mod inputs;
mod flat_mesh;
mod asset_loader;
mod camera;
mod world;
mod states;
mod utils;


use serde::{Serialize, Deserialize};
use std::sync::Mutex;
#[derive(Serialize, Deserialize)]
pub struct CursorState {
    pub state: bool,
}
pub static JS_EVENT_QUEUE: Mutex<Vec<JsEvent>> = Mutex::new(Vec::new());

#[derive(Debug, Serialize, Clone)]
pub struct JsEvent(pub bool);

#[wasm_bindgen]
pub fn send_state_to_js() -> JsValue {
    let state = CursorState{ state: false };
    info!("alert getting states");
    serde_wasm_bindgen::to_value(&state).unwrap()
}

#[wasm_bindgen]
pub fn lock_change_alert(val: JsValue) {
    let state: CursorState = serde_wasm_bindgen::from_value(val).unwrap();
    info!("alert from rust: {}", state.state);
    JS_EVENT_QUEUE.lock().unwrap().push(JsEvent(state.state));
}

#[derive(Component)]
pub struct FocusButtonText;
fn main() {
    App::new()
    .insert_resource(AssetMetaCheck::Never)
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            //mode: bevy::window::WindowMode::Fullscreen,
            //position: WindowPosition::At(IVec2{x: 50, y: 50}),
            //Does nothing in full screen.
            resolution: WindowResolution::new(1920.0/2., 1080.0 /2.0),
            title: "ToHell".to_string(),
            ..default()
        }),
        ..default()
    }))
    .insert_state(GameState::InGame)
    .add_plugins(UtilsPluginGroup)
    .add_plugins(LoadingGamePluginGroup {state: GameState::InGame})
    .add_plugins(GamePluginGroup {state: GameState::InGame})
    .add_systems(Startup, test)
    .add_systems(Update, button_interaction)
    //.add_systems(Update, bevy::window::close_on_esc)
    .run();
}



fn test(
    mut commands: Commands,
){
    commands.spawn(ButtonBundle{
        style:Style{
            width:Val::Percent(20.),
            aspect_ratio:Some(2.),
            align_items:AlignItems::Center,
            justify_content:JustifyContent::Center,
            ..default()
        },
        background_color: BackgroundColor(Color::RED),
        ..default()
    }).with_children(|button| {
        button.spawn((FocusButtonText, TextBundle::from_section("lock", TextStyle{color:Color::DARK_GREEN, font_size:48.0, ..default()})));
    });
}

fn button_interaction(
    mut buttons: Query<(&Interaction, &mut Visibility)>,

    mut window: Query<&mut Window>,
    time: Res<Time>,
    mut cooldown: Local<(f32, bool)>,
) {
    cooldown.0 += time.delta_seconds();

    let mut window =  window.get_single_mut().unwrap();
    
    let mut event_queue = JS_EVENT_QUEUE.lock().unwrap();
    let events = event_queue.drain(..).collect::<Vec<_>>();
    for event in events {
        if !event.0{
            info!("alert cursor event");
            *buttons.single_mut().1 = Visibility::Visible;
            window.cursor = Cursor {
                visible: true,
                grab_mode: bevy::window::CursorGrabMode::None,
                ..default()
            };
        }

    }

    for (interaction, mut visibility) in buttons.iter_mut() {
        match interaction {
            Interaction::Pressed => {
                if cooldown.0 > 1. {
                    cooldown.1 = true;
                    cooldown.0 = 0.;
                    *visibility = Visibility::Hidden;
                    window.cursor = Cursor {
                    visible: false,
                    grab_mode: bevy::window::CursorGrabMode::Confined,
                    ..default()};
                }
            },
            Interaction::Hovered => (),
            Interaction::None =>  (),
        }
    }
}
struct UtilsPluginGroup;
impl PluginGroup for UtilsPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()

        // .add(FrameTimeDiagnosticsPlugin::default())
        // .add(LogDiagnosticsPlugin::default())

        .add(RapierPhysicsPlugin::<NoUserData>::default())
        //.add(RapierDebugRenderPlugin::default())
    }
}

struct LoadingGamePluginGroup<S: States> {
    pub state: S,
}
impl<S: States> PluginGroup for LoadingGamePluginGroup<S> {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
        .add(AssetLoaderPlugin{state: self.state})
    }
}

struct GamePluginGroup<S: States> {
    pub state: S,
}
impl<S: States> PluginGroup for GamePluginGroup<S> {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
        .add(WorldPlugin {state: self.state})
        .add(InputsPlugin)
        //.add(SpawnerPlugin)
        .add(MainCameraPlugin)
    }
}