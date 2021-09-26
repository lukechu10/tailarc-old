use bevy_app::EventReader;
use bevy_ecs::prelude::*;
use bracket_lib::prelude::*;

use crate::components::{Monster, Player, Position, Viewshed};
use crate::gamelog::GameLog;
use crate::map::Map;
use crate::InputEvent;

pub fn monster_ai_system(
    mut input: EventReader<InputEvent>,
    mut map: ResMut<Map>,
    mut game_log: ResMut<GameLog>,
    mut set: QuerySet<(
        Query<&Position, With<Player>>,
        Query<(&mut Viewshed, &mut Position), With<Monster>>,
    )>,
) {
    let player_pos = *set.q0().single().unwrap();

    for _i in input.iter() {
        for (mut viewshed, mut pos) in set.q1_mut().iter_mut() {
            if viewshed
                .visible_tiles
                .contains(&Point::new(player_pos.x, player_pos.y))
            {
                let distance = DistanceAlg::Pythagoras.distance2d(
                    Point::new(pos.x, pos.y),
                    Point::new(player_pos.x, player_pos.y),
                );
                if distance < 1.5 {
                    // Attack goes here
                    game_log.entries.push("Monster shouts insults".to_string());
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
}
