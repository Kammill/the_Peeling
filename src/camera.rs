use bevy::prelude::*;
use bevy_rapier3d::dynamics::Velocity;

use crate::world::Player;


#[derive(Component)]
pub struct MainCamera;
#[derive(Component)]
pub struct TopCamera;

#[derive(Component, Default)]
pub struct CameraRotationVelocity(pub Vec3);

pub struct MainCameraPlugin;
impl Plugin for MainCameraPlugin {
    fn build(&self, app: &mut App){
        
        app
        .add_systems(Update, adjust_camera);
    }
}


fn adjust_camera(
    mut camera_projection: Query<&mut Projection, With<MainCamera>>,
    player_velocity: Query<(&Velocity, &CameraRotationVelocity), With<Player>>,
){

    let mut projection = camera_projection.single_mut();
    let (l_velocity, r_velocity) = player_velocity.single();
    let lin_velocity = l_velocity.linvel;
    let ang_velocity = r_velocity.0;

    let pi = std::f32::consts::PI;

    let mut max = 15.;
    let positif_angle = ang_velocity.x.abs();
    if positif_angle > max {
        max = positif_angle;
    }
    let normal_ang = positif_angle / max;
    let mut ang_fov = pi / 2.0;

    if normal_ang > 0. {
        ang_fov = pi / 2.  + (pi / 4.) * (normal_ang * normal_ang);
    } 

    let lin_fov = pi / 6.0 + (pi / 2.0 - pi / 6.0) * (-0.01 * lin_velocity.length()).exp();

    let fov = (ang_fov + lin_fov) / 2.;

    *projection = Projection::Perspective(PerspectiveProjection {
        near: 0.1,
        far: 40.,
        aspect_ratio: 1.0,
        fov
    });
}