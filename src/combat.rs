use crate::maze::Maze;
use crate::player::Player;
use raylib::prelude::Vector2;

pub fn hits_enemy(
    player: &Player,
    enemy_position: &Vector2,
    maze: &Maze,
    block_size: usize,
) -> bool {
    let dx = enemy_position.x - player.pos.x;
    let dy = enemy_position.y - player.pos.y;
    let distance = (dx * dx + dy * dy).sqrt();
    let enemy_angle = dy.atan2(dx);
    let mut angle_difference = enemy_angle - player.a;

    while angle_difference > std::f32::consts::PI {
        angle_difference -= 2.0 * std::f32::consts::PI;
    }
    while angle_difference < -std::f32::consts::PI {
        angle_difference += 2.0 * std::f32::consts::PI;
    }

    if angle_difference.abs() > 0.13 {
        return false;
    }

    for step in 0..distance as usize {
        let x = player.pos.x + enemy_angle.cos() * step as f32;
        let y = player.pos.y + enemy_angle.sin() * step as f32;
        let column = x as usize / block_size;
        let row = y as usize / block_size;

        if row >= maze.len() || column >= maze[row].len() {
            return false;
        }

        let cell = maze[row][column];
        if cell != ' ' && cell != 'p' && cell != 'g' && cell != 'a' {
            return false;
        }
    }

    true
}
