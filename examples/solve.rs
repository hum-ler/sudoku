use std::path::Path;

use anyhow::{Result, anyhow};

use sudoku::prelude::{solve, write_grid};

fn main() -> Result<()> {
    // Create a simple puzzle, which is a [[u8; 9]; 9] grid. Use 0 to represent blanks.
    let puzzle = [
        [5, 3, 0, 0, 7, 0, 0, 0, 0],
        [6, 0, 0, 1, 0, 5, 0, 0, 0],
        [9, 8, 0, 0, 0, 0, 0, 6, 0],
        [8, 0, 0, 0, 6, 0, 0, 0, 3],
        [4, 0, 0, 8, 0, 3, 0, 0, 1],
        [7, 0, 0, 0, 2, 0, 0, 0, 6],
        [0, 6, 0, 0, 0, 0, 2, 8, 0],
        [0, 0, 0, 4, 0, 9, 0, 0, 5],
        [0, 0, 0, 0, 8, 0, 0, 7, 9],
    ];

    // Solve the puzzle.
    let solutions = solve(puzzle);
    if solutions.is_empty() {
        return Err(anyhow!("No solution."));
    }

    // Print the solutions.
    for solution in solutions {
        write_grid(solution, None::<&Path>, true, " ")?;
    }

    Ok(())
}
