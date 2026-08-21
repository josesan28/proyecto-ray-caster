use crate::maze::Maze;
use raylib::prelude::Vector2;
use std::f32::consts::PI;

pub struct Player {
    pub pos: Vector2,
    pub a: f32,
}

impl Player {
    pub fn from_maze(maze: &Maze, block_size: usize) -> Self {
        for (row, cells) in maze.iter().enumerate() {
            for (column, &cell) in cells.iter().enumerate() {
                if cell == 'p' {
                    return Self {
                        pos: Vector2::new(
                            (column as f32 + 0.5) * block_size as f32,
                            (row as f32 + 0.5) * block_size as f32,
                        ),
                        a: PI / 3.0,
                    };
                }
            }
        }

        Self {
            pos: Vector2::new(block_size as f32 * 1.5, block_size as f32 * 1.5),
            a: PI / 3.0,
        }
    }
}
