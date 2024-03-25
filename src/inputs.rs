
use bevy::{input::mouse::MouseMotion, prelude::*};
use bevy_rapier3d::{control::KinematicCharacterController, dynamics::{ExternalImpulse, Velocity}};

use crate::{camera::MainCamera, world::Player};

pub struct InputsPlugin;

const ROTATION_SPEED: f32 = 0.2;
const MOVE_SPEED: f32 = 0.7;
const GRAVITY: Vec3 = Vec3::new(0., -9.81, 0.);
const DAMPING: f32 = 7.;

#[derive(Default)]
struct Key(pub Option<KeyCode>);

impl Plugin for InputsPlugin {
    fn build(&self, app: &mut App){
        
        app
        .add_event::<EffectEvent>()
        .add_systems(Update, (catch_inputs, effect));
        
    }
}


fn catch_inputs (

    mut camera_transform: Query<&mut Transform, With<MainCamera>>,

    mut character_controller: Query<(&mut KinematicCharacterController, &mut Velocity, &mut ExternalImpulse)>,
    mut mouse_motion : EventReader<MouseMotion>,

    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut cooldown: Local<(f32, u32)>,
    mut dash: Local<(f32, f32)>,
    mut writer: EventWriter<EffectEvent>,
) {
    dash.0 += time.delta_seconds();
    cooldown.0 += time.delta_seconds();
    if cooldown.0 > 5. {
        cooldown.1 = 0;
    }

    // CAMERA
    let mut motion_sum: Vec2 = Vec2::ZERO;
    for motion in mouse_motion.read() {
        motion_sum += motion.delta;
    }

    let mut camera_transform = camera_transform.single_mut();

    let yaw_rotation = -ROTATION_SPEED * motion_sum.x * time.delta_seconds();
    let pitch_rotation = ROTATION_SPEED * motion_sum.y * time.delta_seconds();

    //yaw
    camera_transform.rotate_y(yaw_rotation);

    //pitch
    //TODO: Add Clamping
    let dir = camera_transform.right();
    let mut fake_transform = camera_transform.clone();
    fake_transform.rotate_axis(Vec3::new(dir.x, dir.y, dir.z), -pitch_rotation);

    let foward_dir = fake_transform.forward();
    let dir_vec = Vec3::new(foward_dir.x, foward_dir.y, foward_dir.z);
    let dot = -Vec3::Y.dot(dir_vec);

    if  dot > -0.8 && dot < 0.8 {
        camera_transform.rotate_axis(Vec3::new(dir.x, dir.y, dir.z), -pitch_rotation);
    }

    
    // PLAYER / COLLIDER

    let (mut character_controller, mut velocity, mut external_force) = character_controller.single_mut();

    if keyboard.pressed(KeyCode::Space) {
        if cooldown.0 > 1. {
            
            external_force.impulse += Vec3::Y * 8.;
            cooldown.0 = 0.;
            cooldown.1 += 1;
        }
    }

    // makes moving foward/left not affect the y dimension
    let mut forward:Vec3 = *camera_transform.forward();
    forward.y = 0.;
    let mut left:Vec3 = *camera_transform.left();
    left.y = 0.;

    let mut h_shift = Vec3::ZERO;

    if keyboard.pressed(KeyCode::KeyW) {
        h_shift += forward;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        h_shift -= forward;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        h_shift += left;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        h_shift -= left;
    }


    let mut horizontal_shift = h_shift.normalize_or_zero();

    if keyboard.pressed(KeyCode::KeyR) {
        if dash.0 > 2.5 {
            writer.send(EffectEvent(horizontal_shift, 50.));
            dash.0 = 0.
        }
    }

    let gravity = GRAVITY * time.delta_seconds();
    let mut damping = velocity.linvel;
    damping.x *= -MOVE_SPEED * DAMPING;
    damping.z *= -MOVE_SPEED * DAMPING;
    damping *= time.delta_seconds();
    velocity.linvel += horizontal_shift * MOVE_SPEED + damping + gravity;

    character_controller.translation = Some(velocity.linvel * time.delta_seconds());
}


#[derive(Event, Clone)]
struct EffectEvent(pub Vec3, pub f32);

fn effect(

    time: Res<Time>,


    mut event_set: ParamSet<(
        EventReader<EffectEvent>,
        EventWriter<EffectEvent>,
    )>,

    mut character_velocity: Query<&mut Velocity, With<Player>>,
) {

    let mut events: Vec<EffectEvent> = Vec::new();

    for event in event_set.p0().read() {
        events.push(event.clone());
    }

    for mut event in events {

        let dash = 200. * time.delta_seconds();
        event.1 -= dash;
        let advance = event.0 * dash;
        character_velocity.single_mut().linvel += advance;

        if event.1 > 0. {
            event_set.p1().send(event);
        }

    }

}