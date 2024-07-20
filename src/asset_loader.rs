
use bevy::{asset:: UntypedAssetId, prelude::*, utils::HashMap};


#[derive(Default, Resource)]
pub struct GameAssets{

    requested: HashMap<usize, Vec<UntypedAssetId>>,

    handles: usize,
    image: Vec<(Handle<Image>, String)>,
    meshes: Vec<Handle<Mesh>>,
    materials: Vec<Handle<StandardMaterial>>,
    scenes: Vec<(Handle<Scene>, String)>,
    animations: Vec<(Handle<AnimationClip>, String)>,
}

impl GameAssets {

    pub fn new_world(&mut self) -> usize {
        
        let handle = self.handles;
        self.handles += 1;
        handle
    }

    pub fn request_image(&mut self, world_handle: usize, path: String, server: &Res<AssetServer>) 
        -> Handle<Image> 
    {   

        if !self.requested.contains_key(&world_handle){
            self.requested.insert(world_handle, Vec::new());
        }

        for img in self.image.iter(){
            if img.1 == path {
                return img.0.clone();
            }
        }
        let handle: Handle<Image> = server.load(path.clone());
        self.image.push((handle.clone(), path));
        self.requested.get_mut(&world_handle).unwrap().push(handle.clone().into());
        handle
        
    }

    pub fn request_scene(&mut self, world_handle: usize, path: String, server: &Res<AssetServer>) 
    -> Handle<Scene> 
    {   

        if !self.requested.contains_key(&world_handle){
            self.requested.insert(world_handle, Vec::new());
        }

        for scene in self.scenes.iter(){
            if scene.1 == path {
                return scene.0.clone();
            }
        }
        let handle: Handle<Scene> = server.load(path.clone());
        self.scenes.push((handle.clone(), path));
        self.requested.get_mut(&world_handle).unwrap().push(handle.clone().into());
        handle
    
    }

    pub fn request_animation(&mut self, world_handle: usize, path: String, server: &Res<AssetServer>) 
    -> Handle<AnimationClip> 
    {   

        if !self.requested.contains_key(&world_handle){
            self.requested.insert(world_handle, Vec::new());
        }

        for animation in self.animations.iter(){
            if animation.1 == path {
                return animation.0.clone();
            }
        }
        let handle: Handle<AnimationClip> = server.load(path.clone());
        self.animations.push((handle.clone(), path));
        self.requested.get_mut(&world_handle).unwrap().push(handle.clone().into());
        handle
    
    }

    pub fn wait_for_world_assets(&mut self, world_handle: usize, server: &Res<AssetServer>){
        loop{

            let mut to_remove: Vec<UntypedAssetId> = Vec::new();
            for id in self.requested.get(&world_handle).unwrap().iter() {
    
                let state = server.get_load_state(*id);
                match state {
                    Some(_) => {
                        to_remove.push(*id);
                    },
                    None => (),
                }
            }
            
            for loaded_assets in to_remove {
                let mut idx = 0;
                for id in self.requested.get(&world_handle).unwrap(){
                    if *id == loaded_assets {
                        break;
                    }
                    idx += 1;
                }
                self.requested.get_mut(&world_handle).unwrap().swap_remove(idx);
            }
    
            if self.requested.get(&world_handle).unwrap().is_empty() {
                break;
            }
        }
    }

    pub fn get_animation(&self) -> Handle<AnimationClip> {
        self.animations[0].0.clone()
    }
}

pub struct AssetLoaderPlugin<S: States> {
    pub state: S,
}
impl<S: States> Plugin for AssetLoaderPlugin<S> {
    fn build(&self, app: &mut App){
        app
        .init_resource::<GameAssets>();
    }
}