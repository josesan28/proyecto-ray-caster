use rand::seq::SliceRandom;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub type Maze = Vec<Vec<char>>;

pub fn generate_maze(width: usize, height: usize) -> Maze {
    assert!(width > 0 && height > 0, "El tamaño debe ser mayor que cero");

    let mut maze = vec![vec![' '; width * 3 + 1]; height * 2 + 1];

    for y in 0..=height {
        for x in 0..width {
            maze[y * 2][x * 3] = '+';
            maze[y * 2][x * 3 + 1] = '-';
            maze[y * 2][x * 3 + 2] = '-';
        }
        maze[y * 2][width * 3] = '+';
    }

    for y in 0..height {
        for x in 0..=width {
            maze[y * 2 + 1][x * 3] = '|';
        }
    }

    let mut visited = vec![vec![false; width]; height];
    let mut stack = vec![(0, 0)];
    let mut rng = rand::thread_rng();
    visited[0][0] = true;

    while let Some(&(x, y)) = stack.last() {
        let mut neighbors = Vec::new();

        if x > 0 && !visited[y][x - 1] {
            neighbors.push((x - 1, y));
        }
        if x + 1 < width && !visited[y][x + 1] {
            neighbors.push((x + 1, y));
        }
        if y > 0 && !visited[y - 1][x] {
            neighbors.push((x, y - 1));
        }
        if y + 1 < height && !visited[y + 1][x] {
            neighbors.push((x, y + 1));
        }

        if let Some(&(next_x, next_y)) = neighbors.choose(&mut rng) {
            if next_x != x {
                let wall_x = if next_x > x { (x + 1) * 3 } else { x * 3 };
                maze[y * 2 + 1][wall_x] = ' ';
            } else {
                let wall_y = if next_y > y { (y + 1) * 2 } else { y * 2 };
                maze[wall_y][x * 3 + 1] = ' ';
                maze[wall_y][x * 3 + 2] = ' ';
            }

            visited[next_y][next_x] = true;
            stack.push((next_x, next_y));
        } else {
            stack.pop();
        }
    }

    maze[1][1] = 'p';
    maze[height * 2 - 1][width * 3 - 1] = 'g';
    maze
}

pub fn load_maze(filename: &str) -> io::Result<Maze> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let maze: Maze = reader
        .lines()
        .map(|line| line.map(|line| line.chars().collect()))
        .collect::<io::Result<_>>()?;

    validate_maze(&maze)?;
    Ok(maze)
}

fn validate_maze(maze: &Maze) -> io::Result<()> {
    let Some(first_row) = maze.first() else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "El laberinto está vacío",
        ));
    };

    if first_row.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "El laberinto no puede tener filas vacías",
        ));
    }

    let width = first_row.len();
    if maze.iter().any(|row| row.len() != width) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Todas las filas del laberinto deben tener el mismo ancho",
        ));
    }

    Ok(())
}
