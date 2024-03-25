use bevy::prelude::*;
use bevy_rapier3d::dynamics::Velocity;

use crate::world::Player;


#[derive(Component)]
pub struct MainCamera;

pub struct MainCameraPlugin;
impl Plugin for MainCameraPlugin {
    fn build(&self, app: &mut App){
        
        app
        .add_systems(Update, adjust_camera);
        
    }
}


fn adjust_camera(
    mut camera_projection: Query<&mut Projection, With<MainCamera>>,
    player_velocity: Query<&Velocity, With<Player>>,
){

    let mut projection = camera_projection.single_mut();
    let lin_veclocity = player_velocity.single().linvel;
    

    let pi = std::f64::consts::PI as f32;
    let fov = pi / 6.0 + (pi / 2.0 - pi / 6.0) * (-0.01 * lin_veclocity.length()).exp();

    *projection = Projection::Perspective(PerspectiveProjection {
        near: 0.1,
        far: 40.,
        aspect_ratio: 1.0,
        fov
    });

}