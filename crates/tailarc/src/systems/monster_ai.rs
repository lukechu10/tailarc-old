use bevy_ecs::prelude::*;
use bracket_lib::prelude::*;

use crate::components::{EntityName, Monster, Player, Position, Viewshed, WantsToMelee};
use crate::map::Map;

pub fn monster_ai_system(
    mut commands: Commands,
    mut map: ResMut<Map>,
    mut set: QuerySet<(
        Query<(Entity, &Position), With<Player>>,
        Query<(Entity, &mut Viewshed, &mut Position, &EntityName), With<Monster>>,
    )>,
) {
    let (player_entity, &player_pos) = set.q0().single().unwrap();

    for (entity, mut viewshed, mut pos, _name) in set.q1_mut().iter_mut() {
        if viewshed
            .visible_tiles
            .contains(&Point::new(player_pos.x, player_pos.y))
        {
            let distance = DistanceAlg::Pythagoras.distance2d(
                Point::new(pos.x, pos.y),
                Point::new(player_pos.x, player_pos.y),
            );
            if distance < 1.5 {
                // Within range. Attack the player!
                commands.entity(entity).insert(WantsToMelee {
                    target: player_entity,
                });
                return;
            }
            let path = a_star_search(
                map.xy_idx(pos.x as u32, pos.y as u32),
                map.xy_idx(player_pos.x as u32, player_pos.y as u32),
                &*map,
            );
            if path.success && path.steps.len() > 1 {
                // Move towards the player.

                // Remove the old blocked state because the monster is moving out of that tile.
                let old_idx = map.xy_idx(pos.x as u32, pos.y as u32);
                map.blocked[old_idx] = false;

                pos.x = path.steps[1] as i32 % map.width as i32;
                pos.y = path.steps[1] as i32 / map.width as i32;
                viewshed.dirty = true;

                // Set new blocked state because the monster is now in that tile.
                let new_idx = map.xy_idx(pos.x as u32, pos.y as u32);
                map.blocked[new_idx] = true;
            }
        }
    }
}
