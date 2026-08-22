use crate::maze::Maze;
use crate::player::Player;

pub fn update_objective(
    maze: &mut Maze,
    player: &Player,
    block_size: usize,
    has_artifact: &mut bool,
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

    *has_artifact && maze[row][column] == 'g'
}
