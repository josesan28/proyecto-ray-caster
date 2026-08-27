use crate::game::{GameState, change_mural_digit};
use crate::maze::{Maze, is_walkable_cell};
use crate::player::Player;
use raylib::prelude::{GamepadAxis, GamepadButton, KeyboardKey, RaylibHandle};
use std::f32::consts::PI;

pub const GAMEPAD_ID: i32 = 0;

pub fn gamepad_button_pressed(window: &RaylibHandle, button: GamepadButton) -> bool {
    window.is_gamepad_available(GAMEPAD_ID) && window.is_gamepad_button_pressed(GAMEPAD_ID, button)
}

pub fn process_mural_input(
    window: &mut RaylibHandle,
    state: &mut GameState,
    gamepad_connected: bool,
) -> bool {
    if !state.mural_active {
        state.mural_horizontal_ready = false;
        state.mural_vertical_ready = false;
        return false;
    }

    while let Some(character) = window.get_char_pressed() {
        if character.is_ascii_digit() && state.mural_input.len() < 3 {
            state.mural_input.push(character);
            state.mural_slot = state.mural_input.len().min(3).saturating_sub(1);
        }
    }

    if window.is_key_pressed(KeyboardKey::KEY_BACKSPACE) {
        state.mural_input.pop();
        state.mural_slot = state.mural_input.len().min(3).saturating_sub(1);
    }

    let joystick_x = if gamepad_connected {
        window.get_gamepad_axis_movement(GAMEPAD_ID, GamepadAxis::GAMEPAD_AXIS_LEFT_X)
    } else {
        0.0
    };
    let joystick_y = if gamepad_connected {
        window.get_gamepad_axis_movement(GAMEPAD_ID, GamepadAxis::GAMEPAD_AXIS_LEFT_Y)
    } else {
        0.0
    };

    let joystick_left = state.mural_horizontal_ready && joystick_x < -0.55;
    let joystick_right = state.mural_horizontal_ready && joystick_x > 0.55;
    if joystick_left || joystick_right {
        state.mural_horizontal_ready = false;
    } else if joystick_x.abs() < 0.25 {
        state.mural_horizontal_ready = true;
    }

    let joystick_up = state.mural_vertical_ready && joystick_y < -0.55;
    let joystick_down = state.mural_vertical_ready && joystick_y > 0.55;
    if joystick_up || joystick_down {
        state.mural_vertical_ready = false;
    } else if joystick_y.abs() < 0.25 {
        state.mural_vertical_ready = true;
    }

    if joystick_left {
        state.mural_slot = state.mural_slot.saturating_sub(1);
    }
    if joystick_right {
        state.mural_slot = (state.mural_slot + 1).min(2);
    }
    if joystick_up {
        change_mural_digit(&mut state.mural_input, state.mural_slot, 1);
    }
    if joystick_down {
        change_mural_digit(&mut state.mural_input, state.mural_slot, -1);
    }
    if gamepad_button_pressed(window, GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_RIGHT) {
        state.mural_input.clear();
        state.mural_slot = 0;
    }

    window.is_key_pressed(KeyboardKey::KEY_ENTER)
        || window.is_key_pressed(KeyboardKey::KEY_KP_ENTER)
        || gamepad_button_pressed(window, GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_DOWN)
}

pub fn process_events(window: &RaylibHandle, player: &mut Player, maze: &Maze, block_size: usize) {
    const MOVE_SPEED: f32 = 90.0;
    const ROTATION_SPEED: f32 = PI;
    const MOUSE_SENSITIVITY: f32 = 0.003;
    let delta_time = window.get_frame_time().min(0.05);

    player.a += window.get_mouse_delta().x * MOUSE_SENSITIVITY;

    let mut rotation = 0.0;
    if window.is_key_down(KeyboardKey::KEY_LEFT) || window.is_key_down(KeyboardKey::KEY_A) {
        rotation -= 1.0;
    }
    if window.is_key_down(KeyboardKey::KEY_RIGHT) || window.is_key_down(KeyboardKey::KEY_D) {
        rotation += 1.0;
    }
    if window.is_gamepad_available(GAMEPAD_ID) {
        rotation += apply_deadzone(
            window.get_gamepad_axis_movement(GAMEPAD_ID, GamepadAxis::GAMEPAD_AXIS_RIGHT_X),
        );
    }
    player.a += rotation.clamp(-1.0, 1.0) * ROTATION_SPEED * delta_time;
    player.a = player.a.rem_euclid(2.0 * PI);

    let mut movement_direction = 0.0;
    if window.is_key_down(KeyboardKey::KEY_UP) || window.is_key_down(KeyboardKey::KEY_W) {
        movement_direction += 1.0;
    }
    if window.is_key_down(KeyboardKey::KEY_DOWN) || window.is_key_down(KeyboardKey::KEY_S) {
        movement_direction -= 1.0;
    }
    if window.is_gamepad_available(GAMEPAD_ID) {
        movement_direction -= apply_deadzone(
            window.get_gamepad_axis_movement(GAMEPAD_ID, GamepadAxis::GAMEPAD_AXIS_LEFT_Y),
        );
    }
    let movement = movement_direction.clamp(-1.0, 1.0) * MOVE_SPEED * delta_time;

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

fn apply_deadzone(value: f32) -> f32 {
    const DEADZONE: f32 = 0.20;

    if value.abs() < DEADZONE {
        0.0
    } else {
        value.signum() * (value.abs() - DEADZONE) / (1.0 - DEADZONE)
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

    is_walkable_cell(maze[row][column])
}
