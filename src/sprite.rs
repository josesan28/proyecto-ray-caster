use crate::maze::Maze;
use raylib::prelude::Vector2;
use std::collections::VecDeque;

pub struct Enemy {
    pub pos: Vector2,
    next_cell: Option<(usize, usize)>,
    chase_time: f32,
}

pub struct Artifact {
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
                        next_cell: None,
                        chase_time: 0.0,
                    };
                }
            }
        }

        Self {
            pos: Vector2::new(block_size as f32 * 3.5, block_size as f32 * 3.5),
            next_cell: None,
            chase_time: 0.0,
        }
    }

    pub fn update(
        &mut self,
        player_position: &Vector2,
        maze: &Maze,
        block_size: usize,
        delta_time: f32,
        speed_multiplier: f32,
    ) {
        const INITIAL_SPEED_IN_BLOCKS: f32 = 0.45;
        const MAX_SPEED_IN_BLOCKS: f32 = 1.15;
        const SECONDS_TO_MAX_SPEED: f32 = 90.0;

        self.chase_time += delta_time;

        let distance_to_player = ((player_position.x - self.pos.x).powi(2)
            + (player_position.y - self.pos.y).powi(2))
        .sqrt();
        if distance_to_player <= block_size as f32 * 0.65 {
            self.next_cell = None;
            return;
        }

        let current_cell = position_to_cell(&self.pos, block_size);
        let player_cell = position_to_cell(player_position, block_size);
        if self.next_cell.is_none() {
            self.next_cell = next_step_toward(current_cell, player_cell, maze);
        }

        let Some((target_column, target_row)) = self.next_cell else {
            return;
        };

        let target = Vector2::new(
            (target_column as f32 + 0.5) * block_size as f32,
            (target_row as f32 + 0.5) * block_size as f32,
        );
        let dx = target.x - self.pos.x;
        let dy = target.y - self.pos.y;
        let distance = (dx * dx + dy * dy).sqrt();

        let acceleration_progress = (self.chase_time / SECONDS_TO_MAX_SPEED).min(1.0);
        let speed_in_blocks = INITIAL_SPEED_IN_BLOCKS
            + (MAX_SPEED_IN_BLOCKS - INITIAL_SPEED_IN_BLOCKS) * acceleration_progress;
        let movement = speed_in_blocks * block_size as f32 * speed_multiplier.max(0.0) * delta_time;

        if distance <= movement {
            self.pos = target;
            self.next_cell = None;
        } else if distance > 0.0 {
            self.pos.x += dx / distance * movement;
            self.pos.y += dy / distance * movement;
        }
    }
}

fn position_to_cell(position: &Vector2, block_size: usize) -> (usize, usize) {
    (
        position.x.max(0.0) as usize / block_size,
        position.y.max(0.0) as usize / block_size,
    )
}

fn next_step_toward(
    start: (usize, usize),
    goal: (usize, usize),
    maze: &Maze,
) -> Option<(usize, usize)> {
    if start == goal || maze.is_empty() {
        return None;
    }

    let mut visited: Vec<Vec<bool>> = maze.iter().map(|row| vec![false; row.len()]).collect();
    if start.1 >= maze.len()
        || start.0 >= maze[start.1].len()
        || goal.1 >= maze.len()
        || goal.0 >= maze[goal.1].len()
    {
        return None;
    }

    let mut pending = VecDeque::new();
    visited[start.1][start.0] = true;
    pending.push_back((start, start));

    while let Some(((column, row), first_step)) = pending.pop_front() {
        let neighbors = [
            (column.wrapping_sub(1), row),
            (column + 1, row),
            (column, row.wrapping_sub(1)),
            (column, row + 1),
        ];

        for next in neighbors {
            if next.1 >= maze.len()
                || next.0 >= maze[next.1].len()
                || visited[next.1][next.0]
                || !is_enemy_walkable(maze[next.1][next.0])
            {
                continue;
            }

            let next_first_step = if (column, row) == start {
                next
            } else {
                first_step
            };
            if next == goal {
                return Some(next_first_step);
            }

            visited[next.1][next.0] = true;
            pending.push_back((next, next_first_step));
        }
    }

    None
}

fn is_enemy_walkable(cell: char) -> bool {
    matches!(cell, ' ' | 'p' | 'g' | 'a')
}

impl Artifact {
    pub fn from_maze(maze: &Maze, block_size: usize) -> Self {
        for (row, cells) in maze.iter().enumerate() {
            for (column, &cell) in cells.iter().enumerate() {
                if cell == 'a' {
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
            pos: Vector2::new(block_size as f32 * 2.5, block_size as f32 * 2.5),
        }
    }
}
