use crate::maze::{Maze, is_clue_cell};
use crate::player::Player;
use crate::sprite::Clue;

pub fn collect_clue(
    maze: &mut Maze,
    player: &Player,
    block_size: usize,
    clues: &mut [Clue],
) -> Option<char> {
    let column = player.pos.x as usize / block_size;
    let row = player.pos.y as usize / block_size;

    if row >= maze.len() || column >= maze[row].len() {
        return None;
    }

    let digit = maze[row][column];
    if !is_clue_cell(digit) {
        return None;
    }

    if let Some(clue) = clues
        .iter_mut()
        .find(|clue| clue.digit == digit && !clue.collected)
    {
        clue.collected = true;
    }
    maze[row][column] = ' ';
    Some(digit)
}

pub fn update_objective(
    maze: &mut Maze,
    player: &Player,
    block_size: usize,
    has_artifact: &mut bool,
    has_all_clues: bool,
) -> bool {
    let column = player.pos.x as usize / block_size;
    let row = player.pos.y as usize / block_size;

    if row >= maze.len() || column >= maze[row].len() {
        return false;
    }

    if maze[row][column] == 'a' {
        *has_artifact = true;
        maze[row][column] = ' ';
    }

    *has_artifact && has_all_clues && maze[row][column] == 'g'
}
