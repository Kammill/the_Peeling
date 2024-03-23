
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
pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App){
        app
        .init_resource::<TextureAssets>()
        .init_resource::<MeshAssets>()
        .init_resource::<ColorMaterialAssets>()
        .add_systems(Startup, (load_textures, load_meshes));
    }
}


fn load_textures (
    mut textures: ResMut<TextureAssets>,
    asset_server: Res<AssetServer>,
){
    textures.ground.push(asset_server.load("textures/gravier_16px.png"));
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