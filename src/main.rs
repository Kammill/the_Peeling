use bevy::{app::PluginGroupBuilder, prelude::*, window::{Cursor, WindowResolution}};

use asset_loader::{AssetLoaderPlugin, TextureAssets};
use bevy_rapier3d::{control::{CharacterLength, KinematicCharacterController}, dynamics::{LockedAxes, RigidBody, Velocity}, geometry::Collider, plugin::{NoUserData, RapierPhysicsPlugin}, render::RapierDebugRenderPlugin};
use flat_mesh::gen_flat_mesh;
use inputs::InputsPlugin;
use rand::Rng;

mod inputs;
mod flat_mesh;
mod asset_loader;

const CHUNK_RADIUS: i32 = 5;
const CHUNK_SIZE: i32 = 50;
const EYE: f32 = 2.;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            position: WindowPosition::At(IVec2{x: 800, y: 100}), 
            cursor: Cursor {
                visible: false,
                grab_mode: bevy::window::CursorGrabMode::Confined,
                ..default()
            },
            resolution: WindowResolution::new(800.0, 600.0),
            title: "Oneslegacy".to_string(),
            ..default()
        }),
        ..default()
    }))
    //Rapier
    .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
    .add_plugins(RapierDebugRenderPlugin::default())
    
    .add_plugins(TestPluginGroup)
    .add_systems(Update, bevy::window::close_on_esc)

    .run();
}

struct TestPluginGroup;

impl PluginGroup for TestPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(WorldPlugin)
            .add(InputsPlugin)
            .add(SpawnerPlugin)
    }
}

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {

        app
        .add_plugins(AssetLoaderPlugin)
        .add_systems(Startup, (placement, world_builder, add_spawner).chain());
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerCamera;

pub fn placement (
    mut commands: Commands,

) {
    commands
    .spawn((Player, SpatialBundle{
        transform: Transform::from_xyz(CHUNK_RADIUS as f32 / 2. , EYE / 2., CHUNK_RADIUS as f32 / 2.),
        ..default()
    }))
    .insert(RigidBody::Dynamic)
    .insert(Collider::capsule(Vec3::Y * -0.6, Vec3::Y * 0.6, 0.4))
    .insert(KinematicCharacterController{
        offset: CharacterLength::Relative(0.01),
        up: Vec3::Y,
        ..default()
    })
    // .insert(GravityScale(1.))
    .insert(LockedAxes::ROTATION_LOCKED)
    .insert(Velocity {
        linvel: Vec3::ZERO,
        angvel: Vec3::ZERO
    })
    // .insert(ColliderMassProperties::Density(100.))
    // .insert(ExternalImpulse {
    //     impulse: Vec3::ZERO,
    //     torque_impulse: Vec3::ZERO,
    // })
    .with_children(|player| {
        player.spawn(
            (PlayerCamera, 
                Camera3dBundle{
                    transform: Transform::from_xyz(0., 1., 0.),
                    ..default()
            }
        ));
    });
}

pub fn world_builder (
    mut commands: Commands,
    player: Query<&Transform, With<Player>>,
    textures: Res<TextureAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
){

    let player_position = player.single().translation;
    let chunk_position: (i32, i32) = 
        ((player_position.x / CHUNK_RADIUS as f32).floor() as i32, (player_position.z / CHUNK_RADIUS as f32).floor() as i32);

    let new_mesh = gen_flat_mesh(0, 0, CHUNK_SIZE, CHUNK_SIZE);
    for z in (chunk_position.0 - CHUNK_RADIUS as i32)..(chunk_position.0 + CHUNK_RADIUS as i32) {
        for x in (chunk_position.1 - CHUNK_RADIUS as i32)..(chunk_position.1 + CHUNK_RADIUS as i32) {

            commands.spawn((
            Chunk(x, z),
            PbrBundle{
                transform: Transform::from_xyz((x * CHUNK_SIZE) as f32, 0., (z * CHUNK_SIZE) as f32),
                mesh: meshes.add(new_mesh.clone()),
                material: materials.add(StandardMaterial{
                    base_color_texture: Some(textures.ground[0].clone()),
                    ..default()
                }),
                ..default()
            }))
            .with_children(|ground|{
                ground.spawn(
                    Collider::cuboid(CHUNK_SIZE as f32 / 2., 0.25, CHUNK_SIZE as f32 / 2.)
                ).insert(TransformBundle::from(Transform::from_xyz((CHUNK_SIZE / 2) as f32, -0.5, (CHUNK_SIZE / 2) as f32)));
            });
        }
    }
}

#[derive(Component)]
pub struct Chunk(pub i32, pub i32);


struct SpawnerPlugin;

impl Plugin for SpawnerPlugin {
    fn build(&self, app: &mut App){
        
        app
        .add_systems(Update, (update_spawner, spawning))
        .add_systems(Update, update_mobs);
    }
}

#[derive(Bundle)]
struct SpawnerBundle {

    t: TransformBundle,
    s: Spawner

}

#[derive(Component)]
struct Spawner(pub f32);

#[derive(Component)]
struct Mob;

fn update_mobs(

    mut transforms: ParamSet<(
        Query<&Transform, With<Player>>,
        Query<(&mut Transform, &mut Velocity), With<Mob>>
    )>
){
    let player_position = transforms.p0().single().translation;

    for (mut mob_transform, mut velocity) in transforms.p1().iter_mut() {
        let mob_pos = mob_transform.translation;
        mob_transform.look_to(mob_pos - player_position, Vec3::Y);
        velocity.linvel = Vec3::new(
            mob_transform.forward().x,
            mob_transform.forward().y,
            mob_transform.forward().z
        );
    }
}

fn spawning(
    time: Res<Time>,
    mut commands: Commands,
    mut spawners: Query<(&Transform, &mut Spawner)>
) {
    let mut rng = rand::thread_rng();
    let range = 3.0..5.0;

    for (transform, mut spawner) in spawners.iter_mut() {

        spawner.0 += time.delta_seconds();

        let rng_r = rng.gen_range(range.clone());

        if spawner.0 > rng_r {

            let start = transform.translation;
            let end = start + Vec3::Y * 1.;

            commands.spawn(Mob)
            .insert(TransformBundle{
                local: transform.clone(),
                ..default()
            })
            .insert(RigidBody::Dynamic)
            .insert(Collider::capsule(start, end, 0.25))
            .insert(LockedAxes::ROTATION_LOCKED | LockedAxes::TRANSLATION_LOCKED_Y)
            .insert(Velocity {
                linvel: Vec3::ZERO,
                angvel: Vec3::ZERO
            });

            *spawner = Spawner(0.);
        }
    }
}

fn update_spawner(
    
    mut transforms: ParamSet<(
        Query<&Transform, With<Player>>,
        Query<(&Spawner, &mut Transform)>,
    )>
){
    let mut rng = rand::thread_rng();
    let player_transform = transforms.p0().single().clone();

    for (_, mut transform) in transforms.p1().iter_mut() {
        if transform.translation.distance(player_transform.translation) > 22. {

            let range = 12.0..18.0;

            let radius = rng.gen_range(range);
            let angle:f32 = rng.gen_range(0.0..2.0 *3.1417);
            
            let mut spawner_transform = player_transform.clone();
            spawner_transform.translation.x += radius;
            spawner_transform.rotate_around(player_transform.translation, Quat::from_rotation_y(angle));

            *transform = spawner_transform;
        }
    }
}

fn add_spawner(

    mut commands: Commands,
    player_transform: Query<&Transform, With<Player>>,

){

    info!("add_spawner");
    let mut rng = rand::thread_rng();

    let transform = player_transform.single().clone();
    for _ in 0..5 {

        let range = 12.0..18.0;

        let radius = rng.gen_range(range);
        let angle:f32 = rng.gen_range(0.0..2.0 *3.1417);
        
        let translation = Vec3::X * radius;
        let point = transform.translation + translation;
        let mut spawner_transform = Transform::from_xyz(point.x, point.y, point.z);
        spawner_transform.rotate_around(transform.translation, Quat::from_rotation_y(angle));

        commands.spawn(SpawnerBundle{
            t: TransformBundle{
                local: spawner_transform,
                ..default()
            },
            s: Spawner(0.)
        }); 
    }
}