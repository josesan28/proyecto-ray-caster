use crate::framebuffer::Framebuffer;
use crate::maze::{Maze, is_walkable_cell};
use crate::player::Player;
use raylib::prelude::Color;

pub struct Intersect {
    pub distance: f32,
    pub impact: char,
    pub texture_x: f32,
}

pub fn cast_ray(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    player: &Player,
    angle: f32,
    block_size: usize,
    draw_line: bool,
) -> Intersect {
    let mut distance = 0.0;
    framebuffer.set_current_color(Color::WHITE);

    loop {
        let x = player.pos.x + distance * angle.cos();
        let y = player.pos.y + distance * angle.sin();

        if x < 0.0 || y < 0.0 {
            return Intersect {
                distance,
                impact: '+',
                texture_x: 0.0,
            };
        }

        let x = x as usize;
        let y = y as usize;
        let column = x / block_size;
        let row = y / block_size;

        if row >= maze.len() || column >= maze[row].len() {
            return Intersect {
                distance,
                impact: '+',
                texture_x: 0.0,
            };
        }

        let cell = maze[row][column];
        if !is_walkable_cell(cell) {
            let hit_x = x as f32 / block_size as f32;
            let hit_y = y as f32 / block_size as f32;
            let previous_distance = (distance - 1.0).max(0.0);
            let previous_x = (player.pos.x + previous_distance * angle.cos()) as usize / block_size;

            let texture_x = if previous_x != column {
                hit_y.fract()
            } else {
                hit_x.fract()
            };

            return Intersect {
                distance,
                impact: cell,
                texture_x,
            };
        }

        if draw_line {
            framebuffer.set_pixel(x as u32, y as u32);
        }
        distance += 1.0;
    }
}

pub fn cast_rays(framebuffer: &mut Framebuffer, maze: &Maze, player: &Player, block_size: usize) {
    let num_rays = 60;

    for i in 0..num_rays {
        let progress = i as f32 / (num_rays - 1) as f32;
        let angle = player.a - player.fov / 2.0 + player.fov * progress;
        cast_ray(framebuffer, maze, player, angle, block_size, true);
    }
}
