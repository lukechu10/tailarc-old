use bevy_ecs::prelude::*;
use bracket_lib::prelude::{field_of_view_set, Point};

use crate::components::{Player, Position, Viewshed};
use crate::map::Map;

pub fn visibility_system(
    mut map: ResMut<Map>,
    mut q: Query<(&mut Viewshed, &Position, Option<&Player>)>,
) {
    for (mut viewshed, pos, player) in q.iter_mut() {
        if viewshed.dirty {
            viewshed.visible_tiles.clear();
            viewshed.visible_tiles =
                field_of_view_set(Point::new(pos.x, pos.y), viewshed.range, &*map)
                    .into_iter()
                    .map(|p| Position {
                        x: p.x as u32,
                        y: p.y as u32,
                    })
                    .collect();
            viewshed
                .visible_tiles
                .retain(|p| p.x < map.width && p.y < map.height);

            // Reveal what the player can see.
            if player.is_some() {
                map.visible_tiles.fill(false);
                for pos in &viewshed.visible_tiles {
                    let idx = map.xy_idx(pos.x, pos.y);
                    map.revealed_tiles[idx] = true;
                    map.visible_tiles[idx] = true;
                }
            }

            viewshed.dirty = false;
        }
    }
}
