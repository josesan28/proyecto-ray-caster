use crate::maze::Maze;
use raylib::prelude::Vector2;

pub struct Enemy {
    pub pos: Vector2,
}

impl Enemy {
    pub fn from_maze(maze: &Maze, block_size: usize) -> Self {
        for (row, cells) in maze.iter().enumerate() {
            for (column, &cell) in cells.iter().enumerate() {
                if cell == 'g' {
                    return Self {
                        pos: Vector2::new(
                            (column as f32 + 0.5) * block_size as f32,
                            (row as f32 + 0.5) * block_size as f32,
                        ),
                    };
                }
            }
        }

        Self {
            pos: Vector2::new(block_size as f32 * 3.5, block_size as f32 * 3.5),
        }
    }
}
