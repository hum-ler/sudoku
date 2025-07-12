mod io;
mod sudoku;

pub mod prelude {
    pub use super::{
        io::{read_to_grid, write_grid},
        sudoku::{Grid, Puzzle, Solution, generate, solve, solve_any},
    };
}
