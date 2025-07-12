use std::path::Path;

use anyhow::{Result, anyhow};

use sudoku::prelude::{solve_any, write_grid};

fn main() -> Result<()> {
    // Create a simple puzzle, which is a [[u8; 9]; 9] grid. Use 0 to represent blanks.
    let puzzle = [
        [5, 3, 0, 0, 7, 0, 0, 0, 0],
        [6, 0, 0, 1, 9, 5, 0, 0, 0],
        [9, 8, 0, 0, 0, 0, 0, 6, 0],
        [8, 0, 0, 0, 6, 0, 0, 0, 3],
        [4, 0, 0, 8, 0, 3, 0, 0, 1],
        [7, 0, 0, 0, 2, 0, 0, 0, 6],
        [0, 6, 0, 0, 0, 0, 2, 8, 0],
        [0, 0, 0, 4, 1, 9, 0, 0, 5],
        [0, 0, 0, 0, 8, 0, 0, 7, 9],
    ];

    // Solve the puzzle.
    let solution = solve_any(puzzle).ok_or(anyhow!("No solution."))?;

    // Print the solution.
    write_grid(solution, None::<&Path>, true, " ")?;

    Ok(())
}
