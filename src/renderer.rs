use crate::caster::{cast_ray, cast_rays};
use crate::framebuffer::Framebuffer;
use crate::game::GameState;
use crate::maze::Maze;
use crate::player::Player;
use crate::textures::TextureManager;
use raylib::prelude::{Color, Vector2};

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

fn draw_cell(
    framebuffer: &mut Framebuffer,
    xo: usize,
    yo: usize,
    block_size: usize,
    cell: char,
    floor_color: Color,
) {
    let color = match cell {
        '+' | '-' | '|' | '#' | 'J' | 'S' | 'D' => wall_color(cell),
        'g' => Color::new(218, 186, 255, 255),
        'a' => Color::new(55, 168, 120, 255),
        _ => floor_color,
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
    floor_color: Color,
) -> Vec<f32> {
    let half_height = framebuffer.height as f32 / 2.0;
    let projection_distance = framebuffer.width as f32 / (2.0 * (player.fov / 2.0).tan());

    let floor_start = framebuffer.height as usize / 2;
    let sky_top = Color::new(42, 44, 61, 255);
    let sky_horizon = Color::new(69, 57, 62, 255);
    let sky_height = floor_start.max(1);
    for y in 0..sky_height {
        let progress = y as f32 / sky_height.saturating_sub(1).max(1) as f32;
        let mix =
            |start: u8, end: u8| (start as f32 + (end as f32 - start as f32) * progress) as u8;
        framebuffer.set_current_color(Color::new(
            mix(sky_top.r, sky_horizon.r),
            mix(sky_top.g, sky_horizon.g),
            mix(sky_top.b, sky_horizon.b),
            255,
        ));
        framebuffer.draw_rectangle(0, y, framebuffer.width as usize, 1);
    }

    let floor_height = (framebuffer.height as usize - floor_start).max(1);
    for y in floor_start..framebuffer.height as usize {
        let depth = (y - floor_start) as f32 / floor_height as f32;
        let brightness = 1.05 - depth * 0.22;
        framebuffer.set_current_color(Color::new(
            (floor_color.r as f32 * brightness).min(255.0) as u8,
            (floor_color.g as f32 * brightness).min(255.0) as u8,
            (floor_color.b as f32 * brightness).min(255.0) as u8,
            255,
        ));
        framebuffer.draw_rectangle(0, y, framebuffer.width as usize, 1);
    }

    let num_rays = framebuffer.width as usize;
    let mut z_buffer = vec![f32::INFINITY; num_rays];
    for column in 0..num_rays {
        let progress = column as f32 / (num_rays - 1) as f32;
        let angle = player.a - player.fov / 2.0 + player.fov * progress;
        let intersect = cast_ray(framebuffer, maze, player, angle, block_size, false);

        let distance = intersect.distance.max(1.0);
        let corrected_distance = (distance * (angle - player.a).cos()).max(1.0);
        z_buffer[column] = distance;
        let stake_height = block_size as f32 * projection_distance / corrected_distance;
        let stake_top = half_height - stake_height / 2.0;
        let stake_bottom = half_height + stake_height / 2.0;
        let top = stake_top.max(0.0) as usize;
        let bottom = stake_bottom.min(framebuffer.height as f32) as usize;

        let (texture_width, texture_height) = textures.get_size(intersect.impact);
        let texture_x = (intersect.texture_x * (texture_width - 1) as f32) as u32;
        for y in top..bottom {
            let vertical_position = ((y as f32 - stake_top) / stake_height).clamp(0.0, 1.0);
            let texture_y = (vertical_position * (texture_height - 1) as f32) as u32;
            let color = textures.get_pixel_color(intersect.impact, texture_x, texture_y);
            framebuffer.set_current_color(color);
            framebuffer.set_pixel(column as u32, y as u32);
        }
    }

    z_buffer
}

pub fn render_world(
    framebuffer: &mut Framebuffer,
    state: &GameState,
    textures: &TextureManager,
    current_time: f64,
) {
    framebuffer.clear();
    if state.mode_3d {
        let z_buffer = render_3d(
            framebuffer,
            &state.maze,
            &state.player,
            state.block_size,
            textures,
            state.floor_color,
        );

        let mut visible_sprites = Vec::new();
        visible_sprites.push((&state.mural_position, 'm', 68.0, 0.0));
        if !state.enemy_defeated {
            let enemy_animation = current_time as f32 * 2.2;
            let enemy_scale = 82.0 + enemy_animation.sin() * 2.0;
            let enemy_height = enemy_animation.sin() * 1.5;
            visible_sprites.push((&state.enemy.pos, 'e', enemy_scale, enemy_height));
        }
        if !state.has_artifact {
            let animation_time = current_time as f32;
            let artifact_scale = 55.0 + animation_time.sin() * 4.0;
            let artifact_height = animation_time.sin() * 9.0;
            visible_sprites.push((&state.artifact.pos, 'a', artifact_scale, artifact_height));
        }
        let clue_animation = current_time as f32 * 1.8;
        for (index, clue) in state
            .clues
            .iter()
            .filter(|clue| !clue.collected)
            .enumerate()
        {
            let movement = clue_animation + index as f32 * 1.7;
            visible_sprites.push((
                &clue.pos,
                clue.digit,
                44.0 + movement.sin() * 2.0,
                movement.sin() * 6.0,
            ));
        }

        visible_sprites.sort_by(|left, right| {
            let left_distance =
                (left.0.x - state.player.pos.x).powi(2) + (left.0.y - state.player.pos.y).powi(2);
            let right_distance =
                (right.0.x - state.player.pos.x).powi(2) + (right.0.y - state.player.pos.y).powi(2);
            right_distance.total_cmp(&left_distance)
        });

        for (position, sprite_texture, scale, vertical_offset) in visible_sprites {
            render_sprite(
                framebuffer,
                &state.player,
                position,
                sprite_texture,
                scale,
                vertical_offset,
                textures,
                &z_buffer,
            );
        }
        render_minimap(
            framebuffer,
            &state.maze,
            &state.player,
            (!state.enemy_defeated).then_some(&state.enemy.pos),
            state.block_size,
            state.floor_color,
        );
    } else {
        render_maze(
            framebuffer,
            &state.maze,
            state.block_size,
            state.floor_color,
        );
        cast_rays(framebuffer, &state.maze, &state.player, state.block_size);
        if !state.enemy_defeated {
            render_enemy(framebuffer, &state.enemy.pos, state.block_size);
        }
        render_player(framebuffer, &state.player, state.block_size);
    }
}

pub fn render_sprite(
    framebuffer: &mut Framebuffer,
    player: &Player,
    sprite_position: &Vector2,
    texture: char,
    scale: f32,
    vertical_offset: f32,
    textures: &TextureManager,
    z_buffer: &[f32],
) {
    let sprite_angle = (sprite_position.y - player.pos.y).atan2(sprite_position.x - player.pos.x);
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

    let distance = ((player.pos.x - sprite_position.x).powi(2)
        + (player.pos.y - sprite_position.y).powi(2))
    .sqrt();
    if distance < 20.0 {
        return;
    }

    let sprite_size = (framebuffer.height as f32 / distance) * scale;
    let screen_x = ((angle_diff / player.fov) + 0.5) * framebuffer.width as f32;
    let left = screen_x - sprite_size / 2.0;
    let top = framebuffer.height as f32 / 2.0 - sprite_size / 2.0 + vertical_offset;
    let start_x = left.max(0.0) as usize;
    let start_y = top.max(0.0) as usize;
    let end_x = (left + sprite_size).min(framebuffer.width as f32) as usize;
    let end_y = (top + sprite_size).min(framebuffer.height as f32) as usize;
    let (texture_width, texture_height) = textures.get_size(texture);

    for x in start_x..end_x {
        if x >= z_buffer.len() || distance >= z_buffer[x] {
            continue;
        }

        for y in start_y..end_y {
            let tx = (((x as f32 - left) / sprite_size) * (texture_width - 1) as f32) as u32;
            let ty = (((y as f32 - top) / sprite_size) * (texture_height - 1) as f32) as u32;
            let color = textures.get_pixel_color(texture, tx, ty);

            if color.a > 10 {
                framebuffer.set_current_color(color);
                framebuffer.set_pixel(x as u32, y as u32);
            }
        }
    }
}

pub fn render_maze(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    block_size: usize,
    floor_color: Color,
) {
    for (row_index, row) in maze.iter().enumerate() {
        for (col_index, &cell) in row.iter().enumerate() {
            let xo = col_index * block_size;
            let yo = row_index * block_size;
            draw_cell(framebuffer, xo, yo, block_size, cell, floor_color);
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

pub fn render_enemy(framebuffer: &mut Framebuffer, position: &Vector2, block_size: usize) {
    let enemy_size = (block_size / 2).max(6);
    let half_size = enemy_size as f32 / 2.0;
    let x = (position.x - half_size).max(0.0) as usize;
    let y = (position.y - half_size).max(0.0) as usize;

    framebuffer.set_current_color(Color::new(190, 35, 45, 255));
    framebuffer.draw_rectangle(x, y, enemy_size, enemy_size);
}

pub fn render_minimap(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    player: &Player,
    enemy_position: Option<&Vector2>,
    world_block_size: usize,
    floor_color: Color,
) {
    const MINIMAP_BLOCK: usize = 7;
    const PADDING: usize = 12;

    let map_width = maze.first().map_or(0, Vec::len) * MINIMAP_BLOCK;
    let map_height = maze.len() * MINIMAP_BLOCK;
    if map_width == 0 || map_height == 0 {
        return;
    }

    let origin_x = (framebuffer.width as usize).saturating_sub(map_width + PADDING);
    let origin_y = PADDING;

    framebuffer.set_current_color(Color::new(42, 37, 48, 255));
    framebuffer.draw_rectangle(
        origin_x.saturating_sub(4),
        origin_y.saturating_sub(4),
        map_width + 8,
        map_height + 8,
    );

    for (row_index, row) in maze.iter().enumerate() {
        for (col_index, &cell) in row.iter().enumerate() {
            let color = match cell {
                '#' => Color::new(181, 174, 143, 255),
                'J' => Color::new(70, 110, 64, 255),
                'S' => Color::new(120, 125, 115, 255),
                'D' => Color::new(105, 70, 45, 255),
                'g' => Color::new(218, 186, 255, 255),
                'a' => Color::new(55, 168, 120, 255),
                _ => floor_color,
            };

            framebuffer.set_current_color(color);
            framebuffer.draw_rectangle(
                origin_x + col_index * MINIMAP_BLOCK,
                origin_y + row_index * MINIMAP_BLOCK,
                MINIMAP_BLOCK,
                MINIMAP_BLOCK,
            );
        }
    }

    let player_x =
        origin_x + (player.pos.x / world_block_size as f32 * MINIMAP_BLOCK as f32) as usize;
    let player_y =
        origin_y + (player.pos.y / world_block_size as f32 * MINIMAP_BLOCK as f32) as usize;

    framebuffer.set_current_color(Color::new(121, 85, 58, 255));
    framebuffer.draw_rectangle(player_x.saturating_sub(2), player_y.saturating_sub(2), 5, 5);

    if let Some(position) = enemy_position {
        let enemy_x =
            origin_x + (position.x / world_block_size as f32 * MINIMAP_BLOCK as f32) as usize;
        let enemy_y =
            origin_y + (position.y / world_block_size as f32 * MINIMAP_BLOCK as f32) as usize;
        framebuffer.set_current_color(Color::new(220, 45, 55, 255));
        framebuffer.draw_rectangle(enemy_x.saturating_sub(2), enemy_y.saturating_sub(2), 5, 5);
    }

    framebuffer.set_current_color(Color::new(73, 48, 32, 255));
    for distance in 0..(MINIMAP_BLOCK + 2) {
        let x = player_x as f32 + player.a.cos() * distance as f32;
        let y = player_y as f32 + player.a.sin() * distance as f32;
        if x >= 0.0 && y >= 0.0 {
            framebuffer.set_pixel(x as u32, y as u32);
        }
    }
}
