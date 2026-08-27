use crate::maze::{Maze, is_clue_cell};
use crate::player::Player;
use crate::sprite::{Artifact, Clue, Enemy};
use raylib::prelude::{Color, Vector2};

pub const ENEMY_MAX_HEALTH: i32 = 6;
pub const SHOT_COOLDOWN: f64 = 0.55;

pub struct GameState {
    pub maze: Maze,
    pub player: Player,
    pub enemy: Enemy,
    pub mural_position: Vector2,
    pub artifact: Artifact,
    pub clues: Vec<Clue>,
    pub block_size: usize,
    pub floor_color: Color,
    pub has_artifact: bool,
    pub enemy_defeated: bool,
    pub enemy_health: i32,
    pub player_defeated: bool,
    pub kukulkan_sound_played: bool,
    pub last_shot_time: f64,
    pub last_enemy_hit_time: Option<f64>,
    pub last_clue_collected: Option<(char, f64)>,
    pub mural_input: String,
    pub mural_slot: usize,
    pub mural_horizontal_ready: bool,
    pub mural_vertical_ready: bool,
    pub wrong_code_attempts: i32,
    pub enemy_speed_multiplier: f32,
    pub last_wrong_code_time: Option<f64>,
    pub level_complete: bool,
    pub mode_3d: bool,
    pub mural_active: bool,
}

impl GameState {
    pub fn new(maze: Maze, block_size: usize, floor_color: Color) -> Self {
        let player = Player::from_maze(&maze, block_size);
        let enemy = Enemy::from_maze(&maze, block_size);
        let mural_position = enemy.pos;
        let artifact = Artifact::from_maze(&maze, block_size);
        let clues = Clue::from_maze(&maze, block_size);

        Self {
            maze,
            player,
            enemy,
            mural_position,
            artifact,
            clues,
            block_size,
            floor_color,
            has_artifact: false,
            enemy_defeated: false,
            enemy_health: ENEMY_MAX_HEALTH,
            player_defeated: false,
            kukulkan_sound_played: false,
            last_shot_time: -1.0,
            last_enemy_hit_time: None,
            last_clue_collected: None,
            mural_input: String::new(),
            mural_slot: 0,
            mural_horizontal_ready: false,
            mural_vertical_ready: false,
            wrong_code_attempts: 0,
            enemy_speed_multiplier: 1.0,
            last_wrong_code_time: None,
            level_complete: false,
            mode_3d: true,
            mural_active: false,
        }
    }

    pub fn clues_found(&self) -> usize {
        self.clues.iter().filter(|clue| clue.collected).count()
    }

    pub fn has_all_clues(&self) -> bool {
        self.clues.len() == 3 && self.clues_found() == 3
    }

    pub fn enemy_distance(&self) -> f32 {
        ((self.player.pos.x - self.enemy.pos.x).powi(2)
            + (self.player.pos.y - self.enemy.pos.y).powi(2))
        .sqrt()
    }
}

pub fn change_mural_digit(input: &mut String, slot: usize, change: i32) {
    let mut digits: Vec<char> = input.chars().collect();
    while digits.len() < 3 {
        digits.push('0');
    }

    let current = digits[slot].to_digit(10).unwrap_or(0) as i32;
    let next = (current + change).rem_euclid(10);
    digits[slot] = char::from_digit(next as u32, 10).unwrap_or('0');
    *input = digits.into_iter().collect();
}

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

pub fn open_doors(maze: &mut Maze) {
    for row in maze {
        for cell in row {
            if *cell == 'D' {
                *cell = ' ';
            }
        }
    }
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
