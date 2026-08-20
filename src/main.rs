pub mod maze;

use maze::generate_maze;

fn main() {
    println!("Holaaaa");

    let maze = generate_maze(10, 10);
    println!(
        "Laberinto generado: {} columnas x {} filas",
        maze[0].len(),
        maze.len()
    );

    for row in &maze {
        let line: String = row.iter().collect();
        println!("{line}");
    }
}
