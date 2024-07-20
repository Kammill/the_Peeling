use bevy::prelude::*;
use bevy_rapier3d::{control::{CharacterLength, KinematicCharacterController}, dynamics::{ExternalImpulse, LockedAxes, RigidBody, Velocity}, geometry::Collider};
use rand::Rng;

use crate::{asset_loader::GameAssets, camera::{CameraRotationVelocity, MainCamera, TopCamera}, flat_mesh::gen_flat_mesh};

const CHUNK_RADIUS: i32 = 5;
const CHUNK_SIZE: i32 = 50;
const EYE: f32 = 2.;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Chunk(pub i32, pub i32);

pub struct WorldPlugin<S: States> {
    pub state: S,
}
impl<S: States> Plugin for WorldPlugin<S> {
    fn build(&self, app: &mut App) {

        app
        .add_systems(Startup, (player_placement, world_builder).chain()
        .run_if(in_state(self.state.clone())))
        .add_systems(Update, tmp_anim_sword);
    }
}


pub fn player_placement (
    mut commands: Commands,
    mut assets: ResMut<GameAssets>,
    server: Res<AssetServer>,
) {
    commands
    .spawn((Player, SpatialBundle{
        transform: Transform::from_xyz(CHUNK_SIZE as f32 / 2. , EYE / 2., CHUNK_SIZE as f32 / 2.),
        ..default()
    }))
    .insert(RigidBody::Dynamic)
    .insert(Collider::capsule(Vec3::Y * -0.6, Vec3::Y * 0.6, 0.4))
    .insert(KinematicCharacterController{
        offset: CharacterLength::Relative(0.05),
        up: Vec3::Y,
        ..default()
    })
    .insert(LockedAxes::ROTATION_LOCKED)
    .insert(Velocity {
        linvel: Vec3::ZERO,
        angvel: Vec3::ZERO
    })
    .insert(CameraRotationVelocity::default())
    .insert(ExternalImpulse  {
        impulse: Vec3::ZERO,
        torque_impulse: Vec3::ZERO,
    })
    .with_children(|player| {
        
        let world_handle = assets.new_world();

        

        let mut top_camera_transform = Transform::from_xyz(0., 5., 0.);
        top_camera_transform.rotate_x(-3.1416 / 2.);
        player.spawn((TopCamera,
            Camera3dBundle{
                camera: Camera {
                    is_active: false,
                    ..default()
                },
                    transform: top_camera_transform,
                    projection: Projection::Perspective(PerspectiveProjection {
                        near: 0.1,
                        far: 40.,
                        aspect_ratio: 1.0,
                        fov: 3.1416 / 2.
                    }),
                    ..default()
            }
        ));

        player.spawn(
            (MainCamera, 
                Camera3dBundle{
                    transform: Transform::from_xyz(0., 1., 0.),
                    projection: Projection::Perspective(PerspectiveProjection {
                        near: 0.1,
                        far: 40.,
                        aspect_ratio: 1.0,
                        fov: 3.1416 / 2.
                    }),
                    ..default()
            }
        )).with_children(|camera|{

            camera.spawn(DirectionalLightBundle {
                transform: Transform::from_xyz(0., 50., -20.),
                // directional_light: DirectionalLight {
                //     ..default()
                // },
                ..default()
            });
        });

        

        //assets.wait_for_world_assets(world_handle, &server);
    });

    
    // Light
    
}

fn tmp_anim_sword(
    mut animations: Query<&mut AnimationPlayer, Added<AnimationPlayer>>,
    assets: Res<GameAssets>,
){
    for mut animation in animations.iter_mut() {
        animation.play(assets.get_animation()).set_speed(0.25).repeat();
    }
}

pub fn world_builder (
    mut commands: Commands,
    player: Query<&Transform, With<Player>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut assets: ResMut<GameAssets>,
    server: Res<AssetServer>,
){
    let world_handle = assets.new_world();
    

    let player_position = player.single().translation;
    let chunk_position: (i32, i32) = 
        ((player_position.x / CHUNK_SIZE as f32).floor() as i32, (player_position.z / CHUNK_SIZE as f32).floor() as i32);

    let new_mesh = gen_flat_mesh(0, 0, CHUNK_SIZE, CHUNK_SIZE);
    for z in (chunk_position.0 - CHUNK_RADIUS as i32)..(chunk_position.0 + CHUNK_RADIUS as i32) {
        for x in (chunk_position.1 - CHUNK_RADIUS as i32)..(chunk_position.1 + CHUNK_RADIUS as i32) {

            commands.spawn((
            Chunk(x, z),
            PbrBundle{
                transform: Transform::from_xyz((x * CHUNK_SIZE) as f32, 0., (z * CHUNK_SIZE) as f32),
                mesh: meshes.add(new_mesh.clone()),
                material: materials.add(StandardMaterial{
                    base_color_texture: Some(assets.request_image(world_handle, "textures/gravier_16px.png".to_string(), &server)),
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

    let mut rng = rand::thread_rng();

    for _ in 0..60 {
        
        let range = 10.0..150.0;
        let radius = rng.gen_range(range);


        let angle:f32 = rng.gen_range(0.0..2.0 *3.1417);
        
        let translation = Vec3::X * radius;
        let point = translation;
        let mut stalagmite_transform = Transform::from_xyz(point.x, point.y, point.z);
        stalagmite_transform.rotate_around(Vec3::ZERO, Quat::from_rotation_y(angle));
        //spawner_transform .translation += Vec3::Y;

        commands.spawn(SceneBundle{
            scene: assets.request_scene(world_handle, "models/deco/stalagmite_base.glb#Scene0".to_string(), &server),
            transform: stalagmite_transform,
            ..default()
        }).
        with_children(|stalagmite|{

            stalagmite.spawn(TransformBundle{
                local: Transform::from_xyz(0., 6.5, 0.),
                ..default()
            }).insert(Collider::cone(6.5, 2.));
        });
        
    }

    assets.wait_for_world_assets(world_handle, &server);
}
