use bevy_ecs::prelude::*;
use bracket_lib::prelude::{field_of_view, Point};

use crate::tilemap::TileMap;
use crate::{PlayerPosition, Position};

pub(crate) struct Viewshed {
    pub visible_tiles: Vec<Position<i32>>,
    pub range: i32,
    pub dirty: bool,
}

pub(crate) fn visibility_system(
    mut map: ResMut<TileMap>,
    mut q: Query<(&mut Viewshed, &PlayerPosition)>,
) {
    for (mut viewshed, pos) in q.iter_mut() {
        if viewshed.dirty {
            viewshed.visible_tiles.clear();
            let fov = field_of_view(Point::new(pos.0.x, pos.0.y), viewshed.range, &*map);
            viewshed.visible_tiles = fov
                .into_iter()
                .map(|p| Position { x: p.x, y: p.y })
                .collect();
            viewshed.visible_tiles.retain(|p| {
                p.x >= 0 && p.x < map.width as i32 && p.y >= 0 && p.y < map.height as i32
            });

            // Reveal what the player can see.
            map.visible_tiles.fill(false);
            for pos in &viewshed.visible_tiles {
                let idx = map.xy_idx(pos.x as u32, pos.y as u32);
                map.revealed_tiles[idx] = true;
                map.visible_tiles[idx] = true;
            }

            viewshed.dirty = false;
        }
    }
}
