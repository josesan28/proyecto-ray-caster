use crate::maze::Maze;
use crate::player::Player;
use raylib::prelude::{KeyboardKey, RaylibHandle};
use std::f32::consts::PI;

pub fn process_events(window: &RaylibHandle, player: &mut Player, maze: &Maze, block_size: usize) {
    const MOVE_SPEED: f32 = 90.0;
    const ROTATION_SPEED: f32 = PI;
    let delta_time = window.get_frame_time().min(0.05);

    if window.is_key_down(KeyboardKey::KEY_LEFT) || window.is_key_down(KeyboardKey::KEY_A) {
        player.a -= ROTATION_SPEED * delta_time;
    }
    if window.is_key_down(KeyboardKey::KEY_RIGHT) || window.is_key_down(KeyboardKey::KEY_D) {
        player.a += ROTATION_SPEED * delta_time;
    }
    player.a = player.a.rem_euclid(2.0 * PI);

    let mut movement = 0.0;
    if window.is_key_down(KeyboardKey::KEY_UP) || window.is_key_down(KeyboardKey::KEY_W) {
        movement += MOVE_SPEED * delta_time;
    }
    if window.is_key_down(KeyboardKey::KEY_DOWN) || window.is_key_down(KeyboardKey::KEY_S) {
        movement -= MOVE_SPEED * delta_time;
    }

    let next_x = player.pos.x + movement * player.a.cos();
    let next_y = player.pos.y + movement * player.a.sin();
    let player_radius = block_size as f32 * 0.18;

    if can_walk_to(next_x, player.pos.y, player_radius, maze, block_size) {
        player.pos.x = next_x;
    }
    if can_walk_to(player.pos.x, next_y, player_radius, maze, block_size) {
        player.pos.y = next_y;
    }
}

fn can_walk_to(x: f32, y: f32, radius: f32, maze: &Maze, block_size: usize) -> bool {
    let points = [
        (x - radius, y - radius),
        (x + radius, y - radius),
        (x - radius, y + radius),
        (x + radius, y + radius),
    ];

    points
        .iter()
        .all(|&(point_x, point_y)| is_walkable(point_x, point_y, maze, block_size))
}

fn is_walkable(x: f32, y: f32, maze: &Maze, block_size: usize) -> bool {
    if x < 0.0 || y < 0.0 {
        return false;
    }

    let column = x as usize / block_size;
    let row = y as usize / block_size;
    if row >= maze.len() || column >= maze[row].len() {
        return false;
    }

    let cell = maze[row][column];
    cell == ' ' || cell == 'p' || cell == 'g'
}
