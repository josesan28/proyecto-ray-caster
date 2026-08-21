use crate::caster::cast_ray;
use crate::framebuffer::Framebuffer;
use crate::maze::Maze;
use crate::player::Player;
use raylib::prelude::Color;

fn wall_color(cell: char) -> Color {
    match cell {
        '+' | '-' | '|' => Color::new(107, 142, 91, 255),
        'g' => Color::new(218, 186, 255, 255),
        _ => Color::new(107, 142, 91, 255),
    }
}

fn draw_cell(framebuffer: &mut Framebuffer, xo: usize, yo: usize, block_size: usize, cell: char) {
    let color = match cell {
        '+' | '-' | '|' => wall_color(cell),
        'p' => Color::new(121, 85, 58, 255),
        'g' => Color::new(218, 186, 255, 255),
        _ => Color::new(245, 231, 200, 255),
    };

    framebuffer.set_current_color(color);
    framebuffer.draw_rectangle(xo, yo, block_size, block_size);
}

pub fn render_3d(framebuffer: &mut Framebuffer, maze: &Maze, player: &Player, block_size: usize) {
    let half_height = framebuffer.height as f32 / 2.0;
    let projection_distance = block_size as f32;

    framebuffer.set_current_color(Color::new(188, 220, 238, 255));
    framebuffer.draw_rectangle(
        0,
        0,
        framebuffer.width as usize,
        framebuffer.height as usize,
    );
    framebuffer.set_current_color(Color::new(245, 231, 200, 255));
    framebuffer.draw_rectangle(
        0,
        framebuffer.height as usize / 2,
        framebuffer.width as usize,
        framebuffer.height as usize / 2,
    );

    let num_rays = framebuffer.width as usize;
    for column in 0..num_rays {
        let progress = column as f32 / (num_rays - 1) as f32;
        let angle = player.a - player.fov / 2.0 + player.fov * progress;
        let intersect = cast_ray(framebuffer, maze, player, angle, block_size, false);

        let distance = intersect.distance.max(1.0);
        let stake_height =
            (half_height * projection_distance / distance).min(framebuffer.height as f32);
        let top = (half_height - stake_height / 2.0).max(0.0) as usize;
        let bottom = (half_height + stake_height / 2.0).min(framebuffer.height as f32) as usize;

        framebuffer.set_current_color(wall_color(intersect.impact));
        for y in top..bottom {
            framebuffer.set_pixel(column as u32, y as u32);
        }
    }
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
