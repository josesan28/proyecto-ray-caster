use crate::controller::{gamepad_button_pressed, GAMEPAD_ID};
use raylib::prelude::*;

pub fn select_level(
    window: &mut RaylibHandle,
    thread: &RaylibThread,
    background: &Texture2D,
    _artifact_icon: &Texture2D,
    music: Option<&Music<'_>>,
) -> usize {
    let level_names = ["Nivel 1: Plaza de Zaculeu", "Nivel 2: Patio ceremonial"];
    let mut selected: usize = 0;
    let mut music_enabled = music.is_some();
    let mut joystick_ready = true;

    while !window.window_should_close() {
        if let Some(track) = music {
            track.update_stream();
        }
        let gamepad_connected = window.is_gamepad_available(GAMEPAD_ID);
        let joystick_y = if gamepad_connected {
            window.get_gamepad_axis_movement(GAMEPAD_ID, GamepadAxis::GAMEPAD_AXIS_LEFT_Y)
        } else {
            0.0
        };
        let joystick_up = joystick_ready && joystick_y < -0.55;
        let joystick_down = joystick_ready && joystick_y > 0.55;
        if joystick_up || joystick_down {
            joystick_ready = false;
        } else if joystick_y.abs() < 0.25 {
            joystick_ready = true;
        }

        let select_up = window.is_key_pressed(KeyboardKey::KEY_UP)
            || window.is_key_pressed(KeyboardKey::KEY_W)
            || joystick_up;
        let select_down = window.is_key_pressed(KeyboardKey::KEY_DOWN)
            || window.is_key_pressed(KeyboardKey::KEY_S)
            || joystick_down;
        let confirm = window.is_key_pressed(KeyboardKey::KEY_ENTER)
            || gamepad_button_pressed(window, GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_DOWN);
        let toggle_music = window.is_key_pressed(KeyboardKey::KEY_M)
            || gamepad_button_pressed(window, GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_LEFT);

        if select_up {
            selected = selected.saturating_sub(1);
        }
        if select_down {
            selected = (selected + 1).min(level_names.len() - 1);
        }
        if confirm {
            return selected;
        }
        if toggle_music {
            if let Some(track) = music {
                if music_enabled {
                    track.pause_stream();
                } else {
                    track.resume_stream();
                }
                music_enabled = !music_enabled;
            }
        }
        let gamepad_name = gamepad_connected.then(|| {
            window
                .get_gamepad_name(GAMEPAD_ID)
                .unwrap_or_else(|| "Control".to_string())
        });

        let mut drawing = window.begin_drawing(thread);
        drawing.clear_background(Color::BLACK);
        drawing.draw_texture_pro(
            background,
            Rectangle::new(0.0, 0.0, background.width as f32, background.height as f32),
            Rectangle::new(0.0, 0.0, 800.0, 600.0),
            Vector2::zero(),
            0.0,
            Color::WHITE,
        );
        drawing.draw_rectangle(0, 0, 800, 600, Color::new(12, 20, 18, 165));

        draw_centered_text(
            &mut drawing,
            "EL SECRETO DE ZACULEU",
            42,
            34,
            Color::new(245, 231, 200, 255),
        );
        draw_centered_text(
            &mut drawing,
            "HUEHUETENANGO, GUATEMALA",
            87,
            17,
            Color::new(225, 197, 112, 255),
        );
        draw_centered_text(
            &mut drawing,
            "Explora las ruinas y encuentra el artefacto ceremonial",
            118,
            18,
            Color::new(245, 231, 200, 255),
        );

        drawing.draw_rectangle(112, 163, 576, 404, Color::new(0, 0, 0, 115));
        drawing.draw_rectangle(120, 155, 560, 404, Color::new(24, 32, 28, 245));
        drawing.draw_rectangle_lines(120, 155, 560, 404, Color::new(198, 175, 126, 255));

        draw_centered_text(
            &mut drawing,
            "SELECCIONA UN NIVEL",
            178,
            18,
            Color::new(225, 197, 112, 255),
        );

        for (index, name) in level_names.iter().enumerate() {
            let item_y = 216 + index as i32 * 64;
            let color = if index == selected {
                Color::new(245, 207, 90, 255)
            } else {
                Color::new(235, 229, 209, 255)
            };

            if index == selected {
                drawing.draw_rectangle(150, item_y, 500, 52, Color::new(41, 81, 68, 255));
                drawing.draw_rectangle_lines(150, item_y, 500, 52, Color::new(225, 197, 112, 255));
                drawing.draw_rectangle(159, item_y + 12, 5, 28, Color::new(245, 207, 90, 255));
            } else {
                drawing.draw_rectangle(150, item_y, 500, 52, Color::new(35, 45, 40, 255));
                drawing.draw_rectangle_lines(150, item_y, 500, 52, Color::new(79, 96, 84, 255));
            }

            draw_centered_text(&mut drawing, name, item_y + 14, 22, color);
        }

        drawing.draw_rectangle(150, 351, 500, 1, Color::new(79, 96, 84, 255));

        let navigation_text = if gamepad_connected {
            "W/S o joystick izquierdo: elegir"
        } else {
            "W/S o flechas: elegir"
        };
        draw_centered_text(
            &mut drawing,
            navigation_text,
            374,
            18,
            Color::new(211, 226, 190, 255),
        );
        let confirm_text = if gamepad_connected {
            "Enter o botón X: comenzar"
        } else {
            "Enter: comenzar"
        };
        draw_centered_text(
            &mut drawing,
            confirm_text,
            406,
            18,
            Color::new(245, 207, 90, 255),
        );
        let music_text = if music.is_none() {
            "Música no encontrada"
        } else if music_enabled {
            if gamepad_connected {
                "M o Cuadrado: música activada"
            } else {
                "M: música activada"
            }
        } else if gamepad_connected {
            "M o Cuadrado: música silenciada"
        } else {
            "M: música silenciada"
        };
        draw_centered_text(
            &mut drawing,
            music_text,
            438,
            17,
            Color::new(167, 207, 180, 255),
        );

        drawing.draw_rectangle(150, 474, 500, 1, Color::new(79, 96, 84, 255));

        if let Some(gamepad_name) = gamepad_name {
            draw_centered_text(
                &mut drawing,
                &format!("Control conectado: {gamepad_name}"),
                489,
                16,
                Color::new(94, 182, 139, 255),
            );
            draw_centered_text(
                &mut drawing,
                "Sticks: mover y girar | R2: lanzar | Triángulo: mapa",
                518,
                15,
                Color::new(211, 226, 190, 255),
            );
        } else {
            draw_centered_text(
                &mut drawing,
                "Controles: teclado y mouse",
                489,
                16,
                Color::new(94, 182, 139, 255),
            );
            draw_centered_text(
                &mut drawing,
                "W/S: mover | A/D o mouse: girar | Espacio: lanzar | M: mapa",
                518,
                15,
                Color::new(211, 226, 190, 255),
            );
        }
    }

    0
}

fn draw_centered_text(
    drawing: &mut RaylibDrawHandle<'_>,
    text: &str,
    y: i32,
    font_size: i32,
    color: Color,
) {
    let text_width = drawing.measure_text(text, font_size);
    drawing.draw_text(text, (800 - text_width) / 2, y, font_size, color);
}
