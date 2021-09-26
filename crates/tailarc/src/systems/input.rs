use bevy_app::EventWriter;
use bevy_ecs::prelude::*;
use bracket_lib::prelude::*;

use crate::components::{Player, Position, Viewshed};
use crate::map::Map;
use crate::InputEvent;

/// Get player position.
pub fn player_input_system(bterm: Res<BTerm>) -> (i32, i32) {
    let mut delta_x = 0;
    let mut delta_y = 0;
    if bterm.key == Some(VirtualKeyCode::Left) || bterm.key == Some(VirtualKeyCode::H) {
        delta_x -= 1;
    }
    if bterm.key == Some(VirtualKeyCode::Right) || bterm.key == Some(VirtualKeyCode::L) {
        delta_x += 1;
    }
    if bterm.key == Some(VirtualKeyCode::Up) || bterm.key == Some(VirtualKeyCode::K) {
        delta_y -= 1;
    }
    if bterm.key == Some(VirtualKeyCode::Down) || bterm.key == Some(VirtualKeyCode::J) {
        delta_y += 1;
    }
    if bterm.key == Some(VirtualKeyCode::Y) {
        delta_y -= 1;
        delta_x -= 1;
    }
    if bterm.key == Some(VirtualKeyCode::U) {
        delta_y -= 1;
        delta_x += 1;
    }
    if bterm.key == Some(VirtualKeyCode::B) {
        delta_y += 1;
        delta_x -= 1;
    }
    if bterm.key == Some(VirtualKeyCode::N) {
        delta_y += 1;
        delta_x += 1;
    }
    (delta_x, delta_y)
}

/// Update player position
pub fn update_player_position_system(
    In((delta_x, delta_y)): In<(i32, i32)>,
    map: Res<Map>,
    mut input: EventWriter<InputEvent>,
    mut q: Query<(&mut Position, &mut Viewshed), With<Player>>,
) {
    if (delta_x, delta_y) != (0, 0) {
        for (mut player_position, mut viewshed) in q.iter_mut() {
            let mut new_position = *player_position;
            new_position.x = (new_position.x + delta_x)
                .max(0)
                .min(map.width.saturating_sub(1) as i32);
            new_position.y = (new_position.y + delta_y)
                .max(0)
                .min(map.height.saturating_sub(1) as i32);

            if !map.blocked[map.xy_idx(new_position.x as u32, new_position.y as u32)] {
                *player_position = new_position;
                viewshed.dirty = true;
                input.send(InputEvent);
            }
        }
    }
}
