use crate::framebuffer::Framebuffer;
use crate::line::line;
use raylib::prelude::{Color, Vector2};
use std::collections::HashMap;

/// Dibuja el contorno de un polígono
pub fn draw_polygon(framebuffer: &mut Framebuffer, points: &[Vector2]) {
    let len = points.len();
    if len < 3 {
        return;
    }
    for i in 0..len {
        let j = (i + 1) % len;
        line(framebuffer, points[i], points[j]);
    }
}

/// Rellena un polígono con fill_color y dibuja su contorno con line_color
pub fn fill_polygon(
    framebuffer: &mut Framebuffer,
    points: &[Vector2],
    fill_color: Color,
    line_color: Color,
) {
    let n = points.len();
    if n < 3 {
        return;
    }

    // Construcción de aristas
    let mut edges: Vec<(f32, f32, f32, f32)> = Vec::new();
    for i in 0..n {
        let j = (i + 1) % n;
        let p1 = points[i];
        let p2 = points[j];
        if (p1.y - p2.y).abs() < f32::EPSILON {
            continue;
        }
        let (y_min, y_max, x_at_ymin, dx_dy) = if p1.y < p2.y {
            (p1.y, p2.y, p1.x, (p2.x - p1.x) / (p2.y - p1.y))
        } else {
            (p2.y, p1.y, p2.x, (p1.x - p2.x) / (p1.y - p2.y))
        };
        edges.push((y_min, y_max, x_at_ymin, dx_dy));
    }

    if edges.is_empty() {
        return;
    }

    let y_min_global = edges.iter().map(|e| e.0).fold(f32::INFINITY, f32::min).floor() as i32;
    let y_max_global = edges.iter().map(|e| e.1).fold(f32::NEG_INFINITY, f32::max).ceil() as i32;

    // Tabla de aristas por y_min
    let mut edge_table: HashMap<i32, Vec<(f32, f32, f32)>> = HashMap::new();
    for (ymin, ymax, x, dx) in edges {
        let ymin_int = ymin.round() as i32;
        edge_table.entry(ymin_int).or_insert_with(Vec::new).push((ymax, x, dx));
    }

    // Lista de aristas activas
    let mut aet: Vec<(f32, f32, f32)> = Vec::new();

    // Establecer color de relleno
    framebuffer.set_current_color(fill_color);

    for y in y_min_global..=y_max_global {
        if let Some(entries) = edge_table.get(&y) {
            for &(ymax, x, dx) in entries {
                aet.push((ymax, x, dx));
            }
        }

        // Eliminar aristas cuyo y_max == y 
        aet.retain(|(ymax, _, _)| *ymax > y as f32);

        // Ordenar por coordenada x
        aet.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        // Dibujar entre pares de intersecciones
        let mut i = 0;
        while i + 1 < aet.len() {
            let x1 = aet[i].1.round() as i32;
            let x2 = aet[i + 1].1.round() as i32;
            for x in x1..=x2 {
                framebuffer.set_pixel(x as u32, y as u32);
            }
            i += 2;
        }

        // Actualizar x para la siguiente línea
        for edge in &mut aet {
            edge.1 += edge.2;
        }
    }

    // Dibujar el contorno con el color de línea
    framebuffer.set_current_color(line_color);
    draw_polygon(framebuffer, points);
}