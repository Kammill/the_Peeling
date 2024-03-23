
use bevy::prelude::*;
#[derive(Resource, Debug, Default)]
pub struct TextureAssets {

    pub ground: Vec<Handle<Image>>,
    
}

#[derive(Resource, Debug, Default)]
pub struct MeshAssets {

    pub meshes: Vec<Handle<Mesh>>,
    
}
pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App){
        app
        .init_resource::<TextureAssets>()
        .add_systems(Startup, load_textures);
    }
}


fn load_textures (
    mut textures: ResMut<TextureAssets>,
    asset_server: Res<AssetServer>,
){
    textures.ground.push(asset_server.load("textures/gravier_16px.png"));
}
