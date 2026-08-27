use crate::game::{ENEMY_MAX_HEALTH, GameState};
use raylib::prelude::*;

pub fn draw_hud(
    drawing: &mut RaylibDrawHandle<'_>,
    state: &GameState,
    current_time: f64,
    shot_progress: Option<f32>,
    gamepad_connected: bool,
    screen_width: i32,
) {
    draw_restart_help(drawing, gamepad_connected, screen_width);
    draw_spear_animation(drawing, shot_progress);
    draw_crosshair(drawing, state);
    draw_enemy_health(drawing, state, current_time);
    draw_status(drawing, state);
    draw_clue_notification(drawing, state, current_time, screen_width);
    draw_mural(
        drawing,
        state,
        current_time,
        gamepad_connected,
        screen_width,
    );
    draw_victory_screen(drawing, state);
    draw_defeat_screen(drawing, state, gamepad_connected, screen_width);
}

fn draw_restart_help(
    drawing: &mut RaylibDrawHandle<'_>,
    gamepad_connected: bool,
    screen_width: i32,
) {
    let restart_help = if gamepad_connected {
        "Options: volver al inicio"
    } else {
        "R: volver al inicio"
    };
    let restart_width = drawing.measure_text(restart_help, 16) + 24;
    let restart_x = screen_width - restart_width - 12;
    drawing.draw_rectangle(
        restart_x,
        555,
        restart_width,
        33,
        Color::new(35, 43, 38, 220),
    );
    drawing.draw_rectangle_lines(
        restart_x,
        555,
        restart_width,
        33,
        Color::new(128, 134, 130, 255),
    );
    drawing.draw_text(
        restart_help,
        restart_x + 12,
        564,
        16,
        Color::new(232, 213, 164, 255),
    );
}

fn draw_spear_animation(drawing: &mut RaylibDrawHandle<'_>, shot_progress: Option<f32>) {
    let Some(progress) = shot_progress else {
        return;
    };

    let travel_progress = (progress / 0.86).min(1.0);
    let spear_x = 400.0 + (travel_progress * std::f32::consts::PI).sin() * 3.0;
    let spear_tip_y = 555.0 - 255.0 * travel_progress;
    let spear_base_y = (spear_tip_y + 105.0).min(600.0);

    drawing.draw_line_ex(
        Vector2::new(spear_x, spear_tip_y + 17.0),
        Vector2::new(spear_x, spear_base_y),
        8.0,
        Color::new(91, 55, 32, 255),
    );
    drawing.draw_line_ex(
        Vector2::new(spear_x - 2.0, spear_tip_y + 20.0),
        Vector2::new(spear_x - 2.0, spear_base_y),
        2.0,
        Color::new(190, 128, 65, 255),
    );
    drawing.draw_triangle(
        Vector2::new(spear_x, spear_tip_y),
        Vector2::new(spear_x - 13.0, spear_tip_y + 22.0),
        Vector2::new(spear_x + 13.0, spear_tip_y + 22.0),
        Color::new(221, 192, 117, 255),
    );
    drawing.draw_triangle(
        Vector2::new(spear_x, spear_tip_y + 4.0),
        Vector2::new(spear_x - 7.0, spear_tip_y + 19.0),
        Vector2::new(spear_x + 7.0, spear_tip_y + 19.0),
        Color::new(255, 235, 171, 255),
    );

    let feather_y = (spear_tip_y + 64.0).min(575.0);
    drawing.draw_triangle(
        Vector2::new(spear_x - 3.0, feather_y),
        Vector2::new(spear_x - 22.0, feather_y + 8.0),
        Vector2::new(spear_x - 3.0, feather_y + 16.0),
        Color::new(54, 145, 104, 255),
    );
    drawing.draw_triangle(
        Vector2::new(spear_x + 3.0, feather_y),
        Vector2::new(spear_x + 3.0, feather_y + 16.0),
        Vector2::new(spear_x + 22.0, feather_y + 8.0),
        Color::new(45, 111, 87, 255),
    );

    if progress > 0.86 {
        let impact = (progress - 0.86) / 0.14;
        drawing.draw_circle_lines(
            400,
            300,
            8.0 + impact * 32.0,
            Color::new(255, 225, 120, ((1.0 - impact) * 255.0) as u8),
        );
    }
}

fn draw_crosshair(drawing: &mut RaylibDrawHandle<'_>, state: &GameState) {
    if !state.mode_3d || state.player_defeated || state.level_complete || state.mural_active {
        return;
    }

    let aim_color = Color::new(70, 240, 255, 255);
    drawing.draw_line_ex(
        Vector2::new(386.0, 300.0),
        Vector2::new(414.0, 300.0),
        3.0,
        aim_color,
    );
    drawing.draw_line_ex(
        Vector2::new(400.0, 286.0),
        Vector2::new(400.0, 314.0),
        3.0,
        aim_color,
    );
    drawing.draw_circle_lines(400, 300, 4.0, Color::new(82, 55, 31, 255));
}

fn draw_enemy_health(drawing: &mut RaylibDrawHandle<'_>, state: &GameState, current_time: f64) {
    if state.enemy_defeated {
        return;
    }

    let recently_hit = state
        .last_enemy_hit_time
        .is_some_and(|hit_time| current_time - hit_time < 0.35);
    let health_color = if recently_hit {
        Color::new(255, 130, 95, 255)
    } else {
        Color::new(232, 213, 164, 255)
    };
    drawing.draw_rectangle(10, 70, 295, 38, Color::new(35, 43, 38, 220));
    drawing.draw_rectangle_lines(10, 70, 295, 38, Color::new(128, 134, 130, 255));
    drawing.draw_text(
        &format!(
            "Resistencia de Kukulkán: {}/{}",
            state.enemy_health, ENEMY_MAX_HEALTH
        ),
        20,
        80,
        18,
        health_color,
    );
}

fn draw_status(drawing: &mut RaylibDrawHandle<'_>, state: &GameState) {
    let clues_found = state.clues_found();
    let has_all_clues = state.has_all_clues();
    let objective = if !has_all_clues && state.enemy_defeated {
        format!("Kukulkán vencido: encuentra los fragmentos ({clues_found}/3)")
    } else if !has_all_clues {
        format!("Kukulkán te persigue: encuentra los fragmentos ({clues_found}/3)")
    } else if !state.has_artifact {
        "Código 250 descubierto: busca el artefacto ceremonial".to_string()
    } else {
        "Artefacto y código listos: ve al mural de la salida".to_string()
    };
    let objective_width = drawing.measure_text(&objective, 20) + 20;
    drawing.draw_rectangle(6, 34, objective_width, 34, Color::new(245, 231, 200, 225));
    drawing.draw_rectangle_lines(6, 34, objective_width, 34, Color::new(105, 70, 45, 255));
    drawing.draw_text(&objective, 16, 41, 20, Color::DARKBROWN);

    let discovered_digit = |digit| {
        if state
            .clues
            .iter()
            .any(|clue| clue.digit == digit && clue.collected)
        {
            digit
        } else {
            '?'
        }
    };
    let code_text = format!(
        "Código: {} - {} - {}",
        discovered_digit('2'),
        discovered_digit('5'),
        discovered_digit('0')
    );
    drawing.draw_rectangle(10, 528, 280, 60, Color::new(35, 43, 38, 220));
    drawing.draw_text(
        "Año de fundación de Zaculeu",
        20,
        536,
        17,
        Color::new(232, 213, 164, 255),
    );
    drawing.draw_text(&code_text, 20, 558, 22, Color::new(83, 211, 151, 255));

    if state.wrong_code_attempts > 0 && !state.enemy_defeated {
        let speed_text = format!("Kukulkán acelerado x{:.2}", state.enemy_speed_multiplier);
        let speed_width = drawing.measure_text(&speed_text, 18) + 20;
        drawing.draw_rectangle(6, 494, speed_width, 32, Color::new(245, 231, 200, 225));
        drawing.draw_rectangle_lines(6, 494, speed_width, 32, Color::new(105, 70, 45, 255));
        drawing.draw_text(&speed_text, 16, 501, 18, Color::new(190, 35, 45, 255));
    }
}

fn draw_clue_notification(
    drawing: &mut RaylibDrawHandle<'_>,
    state: &GameState,
    current_time: f64,
    screen_width: i32,
) {
    if state.mural_active {
        return;
    }
    let Some((digit, collected_time)) = state.last_clue_collected else {
        return;
    };
    if current_time - collected_time >= 3.0 {
        return;
    }

    const MESSAGE_FONT_SIZE: i32 = 22;
    const MESSAGE_PADDING: i32 = 20;
    const MESSAGE_BOX_Y: i32 = 140;
    const MESSAGE_BOX_HEIGHT: i32 = 56;
    let message = format!("Fragmento encontrado: número maya {digit}");
    let message_width = drawing.measure_text(&message, MESSAGE_FONT_SIZE);
    let box_width = message_width + MESSAGE_PADDING * 2;
    let box_x = (screen_width - box_width) / 2;

    drawing.draw_rectangle(
        box_x + 4,
        MESSAGE_BOX_Y + 4,
        box_width,
        MESSAGE_BOX_HEIGHT,
        Color::new(20, 25, 22, 120),
    );
    drawing.draw_rectangle(
        box_x,
        MESSAGE_BOX_Y,
        box_width,
        MESSAGE_BOX_HEIGHT,
        Color::new(35, 43, 38, 235),
    );
    drawing.draw_rectangle_lines(
        box_x,
        MESSAGE_BOX_Y,
        box_width,
        MESSAGE_BOX_HEIGHT,
        Color::new(198, 175, 126, 255),
    );
    drawing.draw_text(
        &message,
        box_x + MESSAGE_PADDING,
        MESSAGE_BOX_Y + 17,
        MESSAGE_FONT_SIZE,
        Color::new(255, 225, 142, 255),
    );
}

fn draw_mural(
    drawing: &mut RaylibDrawHandle<'_>,
    state: &GameState,
    current_time: f64,
    gamepad_connected: bool,
    screen_width: i32,
) {
    if !state.mural_active {
        return;
    }

    const PANEL_X: i32 = 120;
    const PANEL_Y: i32 = 165;
    const PANEL_WIDTH: i32 = 560;
    const PANEL_HEIGHT: i32 = 265;

    drawing.draw_rectangle(
        PANEL_X + 6,
        PANEL_Y + 6,
        PANEL_WIDTH,
        PANEL_HEIGHT,
        Color::new(20, 25, 22, 140),
    );
    drawing.draw_rectangle(
        PANEL_X,
        PANEL_Y,
        PANEL_WIDTH,
        PANEL_HEIGHT,
        Color::new(42, 48, 40, 245),
    );
    drawing.draw_rectangle_lines(
        PANEL_X,
        PANEL_Y,
        PANEL_WIDTH,
        PANEL_HEIGHT,
        Color::new(198, 175, 126, 255),
    );

    let title = "Mural ceremonial";
    let title_width = drawing.measure_text(title, 32);
    drawing.draw_text(
        title,
        (screen_width - title_width) / 2,
        PANEL_Y + 20,
        32,
        Color::new(232, 213, 164, 255),
    );

    let instruction = "Ingresa el año revelado por los fragmentos";
    let instruction_width = drawing.measure_text(instruction, 19);
    drawing.draw_text(
        instruction,
        (screen_width - instruction_width) / 2,
        PANEL_Y + 64,
        19,
        Color::new(205, 196, 169, 255),
    );

    for index in 0..3 {
        let slot_x = 286 + index as i32 * 76;
        let digit = state.mural_input.chars().nth(index).unwrap_or('_');
        drawing.draw_rectangle(slot_x, PANEL_Y + 100, 60, 64, Color::new(24, 29, 25, 255));
        drawing.draw_rectangle_lines(
            slot_x,
            PANEL_Y + 100,
            60,
            64,
            if gamepad_connected && index == state.mural_slot {
                Color::new(70, 240, 255, 255)
            } else {
                Color::new(83, 211, 151, 255)
            },
        );
        let digit_text = digit.to_string();
        let digit_width = drawing.measure_text(&digit_text, 38);
        drawing.draw_text(
            &digit_text,
            slot_x + (60 - digit_width) / 2,
            PANEL_Y + 112,
            38,
            Color::new(255, 225, 142, 255),
        );
    }

    let controls = if gamepad_connected {
        "Joystick izquierdo: editar | X: confirmar | Círculo: borrar"
    } else {
        "Números: escribir | Backspace: borrar | Enter: confirmar"
    };
    let controls_width = drawing.measure_text(controls, 16);
    drawing.draw_text(
        controls,
        (screen_width - controls_width) / 2,
        PANEL_Y + 180,
        16,
        Color::new(205, 196, 169, 255),
    );

    let incorrect_code = state
        .last_wrong_code_time
        .is_some_and(|attempt_time| current_time - attempt_time < 3.0);
    let feedback = if incorrect_code {
        format!(
            "Código incorrecto: Kukulkán acelera x{:.2}",
            state.enemy_speed_multiplier
        )
    } else {
        "Escribe los tres dígitos y confirma el código".to_string()
    };
    let feedback_width = drawing.measure_text(&feedback, 19);
    let feedback_color = if incorrect_code {
        Color::new(255, 110, 95, 255)
    } else {
        Color::new(83, 211, 151, 255)
    };
    drawing.draw_text(
        &feedback,
        (screen_width - feedback_width) / 2,
        PANEL_Y + 218,
        19,
        feedback_color,
    );
}

fn draw_victory_screen(drawing: &mut RaylibDrawHandle<'_>, state: &GameState) {
    if !state.level_complete {
        return;
    }

    drawing.draw_rectangle(170, 245, 460, 110, Color::new(245, 231, 200, 235));
    drawing.draw_text("¡Código correcto!", 265, 275, 32, Color::DARKGREEN);
    drawing.draw_text(
        "La puerta se abrió. Escapaste de Kukulkán",
        196,
        320,
        19,
        Color::DARKBROWN,
    );
}

fn draw_defeat_screen(
    drawing: &mut RaylibDrawHandle<'_>,
    state: &GameState,
    gamepad_connected: bool,
    screen_width: i32,
) {
    if !state.player_defeated {
        return;
    }

    const DEFEAT_X: i32 = 125;
    const DEFEAT_Y: i32 = 205;
    const DEFEAT_WIDTH: i32 = 550;
    const DEFEAT_HEIGHT: i32 = 190;

    drawing.draw_rectangle(
        DEFEAT_X + 6,
        DEFEAT_Y + 6,
        DEFEAT_WIDTH,
        DEFEAT_HEIGHT,
        Color::new(20, 12, 12, 150),
    );
    drawing.draw_rectangle(
        DEFEAT_X,
        DEFEAT_Y,
        DEFEAT_WIDTH,
        DEFEAT_HEIGHT,
        Color::new(45, 24, 24, 245),
    );
    drawing.draw_rectangle_lines(
        DEFEAT_X,
        DEFEAT_Y,
        DEFEAT_WIDTH,
        DEFEAT_HEIGHT,
        Color::new(205, 73, 66, 255),
    );

    let defeat_title = "¡Kukulkán te alcanzó!";
    let defeat_title_width = drawing.measure_text(defeat_title, 34);
    drawing.draw_text(
        defeat_title,
        (screen_width - defeat_title_width) / 2,
        DEFEAT_Y + 30,
        34,
        Color::new(255, 126, 104, 255),
    );

    let defeat_message = "No lograste escapar de las ruinas";
    let defeat_message_width = drawing.measure_text(defeat_message, 21);
    drawing.draw_text(
        defeat_message,
        (screen_width - defeat_message_width) / 2,
        DEFEAT_Y + 88,
        21,
        Color::new(232, 213, 190, 255),
    );

    let restart_message = if gamepad_connected {
        "Presiona Options para volver al inicio"
    } else {
        "Presiona R para volver al inicio"
    };
    let restart_width = drawing.measure_text(restart_message, 20);
    drawing.draw_text(
        restart_message,
        (screen_width - restart_width) / 2,
        DEFEAT_Y + 135,
        20,
        Color::new(255, 220, 125, 255),
    );
}
