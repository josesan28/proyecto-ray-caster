use crate::framebuffer::Framebuffer;
use crate::maze::Maze;
use crate::player::Player;
use raylib::prelude::Color;

fn draw_cell(framebuffer: &mut Framebuffer, xo: usize, yo: usize, block_size: usize, cell: char) {
    let color = match cell {
        '+' | '-' | '|' => Color::new(107, 142, 91, 255),
        'p' => Color::new(121, 85, 58, 255),
        'g' => Color::new(218, 186, 255, 255),
        _ => Color::new(245, 231, 200, 255),
    };

    framebuffer.set_current_color(color);
    framebuffer.draw_rectangle(xo, yo, block_size, block_size);
}

pub fn render_maze(framebuffer: &mut Framebuffer, maze: &Maze, block_size: usize) {
    for (row_index, row) in maze.iter().enumerate() {
        for (col_index, &cell) in row.iter().enumerate() {
            let xo = col_index * block_size;
            let yo = row_index * block_size;
            draw_cell(framebuffer, xo, yo, block_size, cell);
        }
    }
}

pub fn render_player(framebuffer: &mut Framebuffer, player: &Player, block_size: usize) {
    let player_size = (block_size / 3).max(4);
    let half_size = player_size as f32 / 2.0;
    let x = (player.pos.x - half_size).max(0.0) as usize;
    let y = (player.pos.y - half_size).max(0.0) as usize;

    framebuffer.set_current_color(Color::new(121, 85, 58, 255));
    framebuffer.draw_rectangle(x, y, player_size, player_size);
}
