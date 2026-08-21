use crate::framebuffer::Framebuffer;
use crate::maze::Maze;
use crate::player::Player;
use raylib::prelude::Color;

pub struct Intersect {
    pub distance: f32,
    pub impact: char,
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
            };
        }

        let cell = maze[row][column];
        if cell != ' ' && cell != 'p' && cell != 'g' {
            return Intersect {
                distance,
                impact: cell,
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
