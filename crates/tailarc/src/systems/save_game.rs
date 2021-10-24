//! Game saving systems.

use bevy_ecs::prelude::*;
use bevy_reflect::TypeRegistryArc;
use bevy_scene::serde::SceneSerializer;
use bevy_scene::DynamicScene;

use crate::components::register_component_types;

pub fn save_game_system(world: &mut World) {
    let type_registry = world.get_resource::<TypeRegistryArc>().unwrap();
    register_component_types(&mut type_registry.write());

    let scene = DynamicScene::from_world(world, type_registry);
    let serializer = SceneSerializer::new(&scene, type_registry);

    let data = serde_json::to_string(&serializer).expect("could not serialize scene into JSON");

    println!("{}", data);
    std::process::exit(0);
}
