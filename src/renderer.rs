use crate::caster::cast_ray;
use crate::framebuffer::Framebuffer;
use crate::maze::Maze;
use crate::player::Player;
use crate::sprite::Enemy;
use crate::textures::TextureManager;
use raylib::prelude::Color;

fn wall_color(cell: char) -> Color {
    match cell {
        '+' | '-' | '|' => Color::new(107, 142, 91, 255),
        '#' => Color::new(181, 174, 143, 255),
        'J' => Color::new(70, 110, 64, 255),
        'S' => Color::new(120, 125, 115, 255),
        'D' => Color::new(105, 70, 45, 255),
        'g' => Color::new(218, 186, 255, 255),
        _ => Color::new(107, 142, 91, 255),
    }
}

fn draw_cell(framebuffer: &mut Framebuffer, xo: usize, yo: usize, block_size: usize, cell: char) {
    let color = match cell {
        '+' | '-' | '|' | '#' | 'J' | 'S' | 'D' => wall_color(cell),
        'p' => Color::new(121, 85, 58, 255),
        'g' => Color::new(218, 186, 255, 255),
        _ => Color::new(245, 231, 200, 255),
    };

    framebuffer.set_current_color(color);
    framebuffer.draw_rectangle(xo, yo, block_size, block_size);
}

pub fn render_3d(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    player: &Player,
    block_size: usize,
    textures: &TextureManager,
) -> Vec<f32> {
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
    let mut z_buffer = vec![f32::INFINITY; num_rays];
    for column in 0..num_rays {
        let progress = column as f32 / (num_rays - 1) as f32;
        let angle = player.a - player.fov / 2.0 + player.fov * progress;
        let intersect = cast_ray(framebuffer, maze, player, angle, block_size, false);

        let distance = intersect.distance.max(1.0);
        z_buffer[column] = distance;
        let stake_height =
            (half_height * projection_distance / distance).min(framebuffer.height as f32);
        let top = (half_height - stake_height / 2.0).max(0.0) as usize;
        let bottom = (half_height + stake_height / 2.0).min(framebuffer.height as f32) as usize;

        let texture_x = (intersect.texture_x * 63.0) as u32;
        for y in top..bottom {
            let texture_y = (((y - top) as f32 / stake_height) * 63.0).min(63.0) as u32;
            let color = textures.get_pixel_color(intersect.impact, texture_x, texture_y);
            framebuffer.set_current_color(color);
            framebuffer.set_pixel(column as u32, y as u32);
        }
    }

    z_buffer
}

pub fn render_sprite(
    framebuffer: &mut Framebuffer,
    player: &Player,
    enemy: &Enemy,
    textures: &TextureManager,
    z_buffer: &[f32],
) {
    let sprite_angle = (enemy.pos.y - player.pos.y).atan2(enemy.pos.x - player.pos.x);
    let mut angle_diff = sprite_angle - player.a;

    while angle_diff > std::f32::consts::PI {
        angle_diff -= 2.0 * std::f32::consts::PI;
    }
    while angle_diff < -std::f32::consts::PI {
        angle_diff += 2.0 * std::f32::consts::PI;
    }

    if angle_diff.abs() > player.fov / 2.0 {
        return;
    }

    let distance =
        ((player.pos.x - enemy.pos.x).powi(2) + (player.pos.y - enemy.pos.y).powi(2)).sqrt();
    if distance < 20.0 {
        return;
    }

    let sprite_size = (framebuffer.height as f32 / distance) * 70.0;
    let screen_x = ((angle_diff / player.fov) + 0.5) * framebuffer.width as f32;
    let left = screen_x - sprite_size / 2.0;
    let top = framebuffer.height as f32 / 2.0 - sprite_size / 2.0;
    let start_x = left.max(0.0) as usize;
    let start_y = top.max(0.0) as usize;
    let end_x = (left + sprite_size).min(framebuffer.width as f32) as usize;
    let end_y = (top + sprite_size).min(framebuffer.height as f32) as usize;

    for x in start_x..end_x {
        if x >= z_buffer.len() || distance >= z_buffer[x] {
            continue;
        }

        for y in start_y..end_y {
            let tx = (((x as f32 - left) / sprite_size) * 63.0) as u32;
            let ty = (((y as f32 - top) / sprite_size) * 63.0) as u32;
            let color = textures.get_pixel_color('e', tx, ty);

            if color != Color::new(152, 0, 136, 255) {
                framebuffer.set_current_color(color);
                framebuffer.set_pixel(x as u32, y as u32);
            }
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
