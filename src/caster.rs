use crate::framebuffer::Framebuffer;
use crate::maze::Maze;
use crate::player::Player;
use raylib::prelude::Color;

pub fn cast_ray(framebuffer: &mut Framebuffer, maze: &Maze, player: &Player, block_size: usize) {
    let mut distance = 0.0;
    framebuffer.set_current_color(Color::WHITE);

    loop {
        let x = player.pos.x + distance * player.a.cos();
        let y = player.pos.y + distance * player.a.sin();

        if x < 0.0 || y < 0.0 {
            break;
        }

        let x = x as usize;
        let y = y as usize;
        let column = x / block_size;
        let row = y / block_size;

        if row >= maze.len() || column >= maze[row].len() {
            break;
        }

        let cell = maze[row][column];
        if cell != ' ' && cell != 'p' && cell != 'g' {
            break;
        }

        framebuffer.set_pixel(x as u32, y as u32);
        distance += 1.0;
    }
}
