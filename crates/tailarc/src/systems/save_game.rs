//! Game saving systems.

use std::fs;
use std::path::{Path, PathBuf};

use bevy_ecs::prelude::*;
use bevy_reflect::TypeRegistryArc;
use bevy_scene::serde::SceneSerializer;
use bevy_scene::DynamicScene;
use directories::ProjectDirs;

use crate::components::register_component_types;

pub fn save_game_system(world: &mut World) {
    let type_registry = world.get_resource::<TypeRegistryArc>().unwrap();
    register_component_types(&mut type_registry.write());

    let scene = DynamicScene::from_world(world, type_registry);
    let serializer = SceneSerializer::new(&scene, type_registry);

    let data = serde_json::to_string(&serializer).expect("could not serialize scene into JSON");

    let project_dirs = ProjectDirs::from("", "", "Tailarc").expect("could not create ProjectDirs");
    let data_dir = project_dirs.data_dir();

    fs::create_dir_all(data_dir).expect("could not create data directory");
    let data_path = [data_dir, Path::new("save.json")]
        .into_iter()
        .collect::<PathBuf>();
    fs::write(data_path, data).expect("could not write save data");
    std::process::exit(0);
}
