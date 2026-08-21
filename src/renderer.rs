use crate::framebuffer::Framebuffer;
use crate::maze::Maze;
use raylib::prelude::Color;

fn draw_cell(framebuffer: &mut Framebuffer, xo: usize, yo: usize, block_size: usize, cell: char) {
    let color = match cell {
        '+' => Color::BLUE,
        '-' => Color::BLUE,
        '|' => Color::GREEN,
        'p' => Color::BLUE,
        'g' => Color::BLUE,
        _ => Color::RAYWHITE,
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
