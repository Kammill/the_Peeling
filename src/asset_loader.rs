
use bevy::prelude::*;
#[derive(Resource, Debug, Default)]
pub struct TextureAssets {

    pub ground: Vec<Handle<Image>>,
    
}

#[derive(Resource, Debug, Default)]
pub struct MeshAssets {

    pub meshes: Vec<Handle<Mesh>>,
    
}

#[derive(Resource, Debug, Default)]
pub struct ColorMaterialAssets {
    pub materials: Vec<Handle<StandardMaterial>>,
}

#[derive(Resource, Debug, Default)]
pub struct ScenesAssets {
    pub scenes: Vec<Handle<Scene>>,
}

#[derive(Resource, Debug, Default)]
pub struct AnimationAssets {
    pub animations: Vec<Handle<AnimationClip>>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App){
        app
        .init_resource::<TextureAssets>()
        .init_resource::<MeshAssets>()
        .init_resource::<ColorMaterialAssets>()
        .init_resource::<ScenesAssets>()
        .init_resource::<AnimationAssets>()
        .add_systems(Startup, (load_textures, load_meshes, load_scenes, load_animations));
    }
}


fn load_textures (
    mut textures: ResMut<TextureAssets>,
    asset_server: Res<AssetServer>,
){
    textures.ground.push(asset_server.load("textures/gravier_16px.png"));
    //textures.ground.push(asset_server.load("textures/8x8_hellish_palette.png"));
}


fn load_scenes (
    mut scenes: ResMut<ScenesAssets>,
    asset_server: Res<AssetServer>,
){
    scenes.scenes.push(asset_server.load("models/spike_ling_0.glb#Scene0"));
    scenes.scenes.push(asset_server.load("models/test_runner.glb#Scene0"));
}

fn load_animations(
    mut animations: ResMut<AnimationAssets>,
    asset_server: Res<AssetServer>,
){
    animations.animations.push(asset_server.load("models/spike_ling_0.glb#Animation1"));
    animations.animations.push(asset_server.load("models/test_runner.glb#Animation1"));
}


fn load_meshes(
    mut materials: ResMut<ColorMaterialAssets>,
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<MeshAssets>,
    asset_server: Res<AssetServer>,
){

    meshes.meshes.push(asset_server.load("models/shapes/simple_sphere.glb#Mesh0/Primitive0"));

    materials.materials.push(standard_materials.add(StandardMaterial{
        base_color: Color::RED,
        ..default()
    }));

    materials.materials.push(standard_materials.add(StandardMaterial{
        base_color: Color::BLUE,
        ..default()
    }));
}