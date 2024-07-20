

#[derive(Component)]
pub struct TestStart(pub usize);

struct SpawnerPlugin;

impl Plugin for SpawnerPlugin {
    fn build(&self, app: &mut App){
        
        app
        .init_resource::<EntityTypes>()
        .add_systems(Update, update_spawner)
        //.add_systems(Update, (spawning, update_mobs, tmp_animation))
        .add_systems(Update, (link_animations, map_entity_to_type));
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
        Query<(&mut Transform, &mut Velocity, &mut Sleeping, &mut Visibility), With<Mob>>
    )>,
){
    let player_position = transforms.p0().single().translation;
    for (mut mob_transform, mut velocity, mut sleeping, mut visibility) in transforms.p1().iter_mut() {
        
        if player_position.distance(mob_transform.translation) > 50. {
            *visibility = Visibility::Hidden;
            sleeping.sleeping = true;
            //*material = materials.materials[1].clone();
        } else if sleeping.sleeping {
            sleeping.sleeping = false;
            *visibility = Visibility::Visible;
            //*material = materials.materials[0].clone();
        }

        if !sleeping.sleeping {
            mob_transform.look_at(player_position, Vec3::Y);
            let mut delta = player_position - mob_transform.translation;
            delta.y = 0.;
            velocity.linvel = delta.normalize() * 3.;
        }
    }
}


#[derive(Default, Resource)]
struct EntityTypes(pub HashMap<Entity, EntityType>);

enum EntityType {
    Player,
    Ling,
}

fn map_entity_to_type(
    mut entities: ParamSet<(
        Query<(Entity, &Player), Added<Player>>,
        Query<(Entity, &Ling), Added<Ling>>,
    )>,
    mut entity_type_map: ResMut<EntityTypes>,
){
    for (entity, _) in entities.p0().iter() {
        entity_type_map.0.insert(entity, EntityType::Player);
    }

    for (entity, _) in entities.p1().iter() {
        entity_type_map.0.insert(entity, EntityType::Player);
    }
}

#[derive(Component)]
pub struct AnimationEntityLink(pub Entity);

fn get_top_parent(
    mut curr_entity: Entity,
    parent_query: &Query<&Parent>
) -> Entity {
    //Loop up all the way to the top parent
    loop {
        if let Ok(parent) = parent_query.get(curr_entity) {
            curr_entity = parent.get();
        } else {
            break;
        }
    }
    curr_entity
}

pub fn link_animations(
    player_query: Query<Entity, Added<AnimationPlayer>>,
    parent_query: Query<&Parent>,
    animations_entity_link_query: Query<&AnimationEntityLink>,
    mut commands: Commands,
) {
    // Get all the Animation players which can be deep and hidden in the heirachy
    for entity in player_query.iter() {
        let top_entity = get_top_parent(entity, &parent_query);

        // If the top parent has an animation config ref then link the player to the config
        if animations_entity_link_query.get(top_entity).is_ok() {
            warn!("Problem with multiple animationsplayers for the same top parent");
        } else {

            commands
                .entity(top_entity)
                .insert(AnimationEntityLink(entity.clone()));
        }
    }
}

fn tmp_animation(
    mut player_query: Query<&mut AnimationPlayer>,
    query: Query<(Entity, &AnimationEntityLink), Added<AnimationEntityLink>>,
    animations: Res<AnimationAssets>,
    entity_type_map: Res<EntityTypes>,
){
    for (top_entity, player_entity) in query.iter() {
        match entity_type_map.0.get(&top_entity){
            Some(e_type) => match e_type{
                EntityType::Player => {
                    if let Ok(mut player) = player_query.get_mut(player_entity.0){
                        player.play(animations.animations[1].clone_weak()).set_speed(0.8).repeat();
                    }
                }
                EntityType::Ling => {
                    if let Ok(mut player) = player_query.get_mut(player_entity.0){
                        player.play(animations.animations[0].clone_weak()).set_speed(2.).repeat();
                    }
                }
            },
            None => (),
        }
    }
}

fn spawning(
    time: Res<Time>,
    mut commands: Commands,
    mut spawners: Query<(&Transform, &mut Spawner, &Sleeping)>,
    mut mobs: Local<(f32, u32)>,

    // materials: ResMut<ColorMaterialAssets>,
    // meshes: ResMut<MeshAssets>,
    scenes: ResMut<ScenesAssets>,
) {
    mobs.0 += time.delta_seconds();


    let t = Transform::from_scale(Vec3::ONE * 100.);

    let mut rng = rand::thread_rng();
    let range = 3.0..5.0;

    for (transform, mut spawner, sleeping) in spawners.iter_mut() {
        if !sleeping.sleeping
        {
            spawner.0 += time.delta_seconds();

            let rng_r = rng.gen_range(range.clone());
    
            if spawner.0 > rng_r {
    
                let mut count = 0;
                let radius: f32 = 0.5;
                for _ in 0..4 {
                    let mut t =  transform.clone();
                    t.translation += Vec3::X * 1.1 * count as f32 * radius;
                    commands.spawn(Mob)
                    .insert(SceneBundle {
                        scene: scenes.scenes[0].clone(),
                        transform: t,
                        visibility: Visibility::Hidden,
                        ..default()
                    })
                    .insert(TestStart(0))
                    .insert(RigidBody::Dynamic)
                    .insert(Collider::ball(radius))
                    .insert(LockedAxes::ROTATION_LOCKED | LockedAxes::TRANSLATION_LOCKED_Y)
                    .insert(Velocity {
                        linvel: Vec3::ZERO,
                        angvel: Vec3::ZERO
                    })
                    .insert(Sleeping {
                        sleeping: true,
                        ..default()
                    });
                    count += 1;
                    
                }
                mobs.1 += 4;
                *spawner = Spawner(0.);
            }
        }
        }
        
    if mobs.0 > 3. {
        info!("Mobs: {}", mobs.1);
        mobs.0 -= 5.;
    }
    
}

fn update_spawner(
    
    mut transforms: ParamSet<(
        Query<&Transform, With<Player>>,
        Query<(&Spawner, &mut Transform, &mut Sleeping)>,
    )>
){
    let mut player_transform = transforms.p0().single().clone();
    player_transform.translation.y = 1.;
    let player_position = player_transform.translation;

    for (_, spawner_transform, mut sleeping) in transforms.p1().iter_mut(){
        if player_position.distance(spawner_transform.translation) > 100. {
            sleeping.sleeping = true;
        } else if sleeping.sleeping {
            sleeping.sleeping = false;
        }
    }



    // for (_, mut transform) in transforms.p1().iter_mut() {
    //     if transform.translation.distance(player_transform.translation) > 40. {

    //         let range = 35.0..45.;

    //         let radius = rng.gen_range(range);
    //         let angle:f32 = rng.gen_range(0.0..2.0 *3.1417);
            
    //         let mut spawner_transform = player_transform.clone();
    //         spawner_transform.translation.x += radius;
    //         spawner_transform.rotate_around(player_transform.translation, Quat::from_rotation_y(angle));

    //         *transform = spawner_transform;
    //     }
    // }
}

fn add_spawner(

    mut commands: Commands,
    player_transform: Query<&Transform, With<Player>>,

){
    let mut rng = rand::thread_rng();

    let transform = player_transform.single().clone();
    for _ in 0..200 {
        let mut sleeping = true;
        let range = 20.0..150.0;
        let radius = rng.gen_range(range);

        if radius > 75. {
            sleeping = true;
        }

        let angle:f32 = rng.gen_range(0.0..2.0 *3.1417);
        
        let translation = Vec3::X * radius;
        let point = transform.translation + translation;
        let mut spawner_transform = Transform::from_xyz(point.x, point.y, point.z);
        spawner_transform.rotate_around(transform.translation, Quat::from_rotation_y(angle));
        //spawner_transform .translation += Vec3::Y;

        commands.spawn(SpawnerBundle{
            t: TransformBundle{
                local: spawner_transform,
                ..default()
            },
            s: Spawner(0.)
        }).
        insert(Collider::ball(0.1))
        .insert(Sleeping{
            sleeping,
            ..default()
        }); 
    }
}