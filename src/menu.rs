use crate::controller::{GAMEPAD_ID, gamepad_button_pressed};
use raylib::prelude::*;

pub fn select_level(
    window: &mut RaylibHandle,
    thread: &RaylibThread,
    background: &Texture2D,
    artifact_icon: &Texture2D,
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
        drawing.draw_rectangle(0, 0, 800, 600, Color::new(16, 28, 23, 65));
        drawing.draw_rectangle(195, 205, 550, 295, Color::new(24, 35, 31, 225));
        drawing.draw_rectangle_lines(195, 205, 550, 295, Color::new(202, 165, 72, 255));

        drawing.draw_text(
            "EL SECRETO DE ZACULEU",
            177,
            62,
            36,
            Color::new(42, 30, 18, 255),
        );
        drawing.draw_text(
            "EL SECRETO DE ZACULEU",
            175,
            60,
            36,
            Color::new(245, 231, 200, 255),
        );
        drawing.draw_text(
            "HUEHUETENANGO, GUATEMALA",
            268,
            108,
            18,
            Color::new(225, 197, 112, 255),
        );
        drawing.draw_text(
            "Explora las ruinas y encuentra el artefacto ceremonial",
            184,
            145,
            20,
            Color::new(245, 231, 200, 255),
        );

        drawing.draw_texture_pro(
            artifact_icon,
            Rectangle::new(
                0.0,
                0.0,
                artifact_icon.width as f32,
                artifact_icon.height as f32,
            ),
            Rectangle::new(42.0, 265.0, 130.0, 130.0),
            Vector2::zero(),
            0.0,
            Color::WHITE,
        );

        for (index, name) in level_names.iter().enumerate() {
            let item_y = 270 + index as i32 * 55;
            let color = if index == selected {
                Color::new(245, 207, 90, 255)
            } else {
                Color::new(225, 218, 190, 255)
            };

            if index == selected {
                drawing.draw_rectangle(230, item_y - 8, 480, 42, Color::new(41, 81, 68, 235));
                drawing.draw_rectangle_lines(
                    230,
                    item_y - 8,
                    480,
                    42,
                    Color::new(94, 182, 139, 255),
                );
            }

            let prefix = if index == selected { "> " } else { "  " };
            drawing.draw_text(&format!("{prefix}{name}"), 255, item_y, 24, color);
        }

        let navigation_text = if gamepad_connected {
            "W/S o joystick izquierdo: elegir"
        } else {
            "W/S o flechas: elegir"
        };
        drawing.draw_text(
            navigation_text,
            340,
            410,
            18,
            Color::new(211, 226, 190, 255),
        );
        let confirm_text = if gamepad_connected {
            "Enter o botón X: comenzar"
        } else {
            "Enter: comenzar"
        };
        drawing.draw_text(confirm_text, 340, 444, 18, Color::new(245, 207, 90, 255));
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
        drawing.draw_text(music_text, 340, 474, 17, Color::new(167, 207, 180, 255));

        if let Some(gamepad_name) = gamepad_name {
            drawing.draw_text(
                &format!("Control conectado: {gamepad_name}"),
                230,
                520,
                16,
                Color::new(94, 182, 139, 255),
            );
            drawing.draw_text(
                "Sticks: mover y girar | R2: lanzar | Triángulo: mapa",
                205,
                545,
                16,
                Color::new(211, 226, 190, 255),
            );
        }
    }

    0
}
