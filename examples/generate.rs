use std::path::Path;

use anyhow::Result;

use sudoku::prelude::{generate, write_grid};

fn main() -> Result<()> {
    // Generate a simple puzzle, which is a [[u8; 9]; 9] grid, where 0 is used to represent blanks.
    let puzzle = generate();

    // Print the puzzle.
    write_grid(puzzle, None::<&Path>, true, " ")?;

    Ok(())
}
