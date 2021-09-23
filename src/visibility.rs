use bevy_ecs::prelude::*;

use crate::tile::TileMap;
use crate::Position;

pub(crate) struct Viewshed {
    pub visible_tiles: Vec<Position<i32>>,
    pub range: i32,
}

pub(crate) fn visibility_system(
    map: Res<TileMap>,
    mut q: Query<(&mut Viewshed, &mut Position<i32>)>,
) {
    for (mut viewshed, pos) in q.iter_mut() {
        viewshed.visible_tiles.clear();
        // viewshed.visible_tiles = field_of_view(*pos, viewshed.range, &map);
    }
}
