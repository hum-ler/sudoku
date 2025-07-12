use std::path::PathBuf;

use anyhow::{Result, anyhow};
use clap::{Parser, Subcommand};

use sudoku::prelude::*;

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Generates a puzzle.
    Gen {
        /// The output file to write to (overwrites), omit to write to stdout.
        #[arg(short, long = "output")]
        output_file: Option<PathBuf>,

        /// Do not draw border to format the puzzle.
        #[arg(short = 'n', long = "no-border")]
        plain_output: bool,

        /// The character that represents a blank space.
        #[arg(short = 'b', long = "blank", default_value_t = ' ')]
        blank_char: char,
    },
    /// Solves a puzzle.
    Solve {
        /// The input file to read from, omit to read from stdin.
        #[arg(short, long = "input")]
        input_file: Option<PathBuf>,

        /// The output file to write to (overwrites), omit to write to stdout.
        #[arg(short, long = "output")]
        output_file: Option<PathBuf>,

        /// Do not draw border to format the solution.
        #[arg(short = 'n', long = "no-border")]
        plain_output: bool,
    },
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Command::Gen {
            output_file,
            plain_output,
            blank_char,
        } => gen_command(output_file, plain_output, blank_char)?,
        Command::Solve {
            input_file,
            output_file,
            plain_output,
        } => solve_command(input_file, output_file, plain_output)?,
    }

    Ok(())
}

/// Executes the gen command.
fn gen_command(output_file: Option<PathBuf>, plain_output: bool, blank_char: char) -> Result<()> {
    let puzzle = generate();

    write_grid(puzzle, output_file, !plain_output, &blank_char.to_string())
}

/// Executes the solve command.
fn solve_command(
    input_file: Option<PathBuf>,
    output_file: Option<PathBuf>,
    plain_output: bool,
) -> Result<()> {
    let puzzle = read_to_grid(input_file)?;
    let solution = solve_any(puzzle).ok_or(anyhow!("No solution."))?;

    write_grid(solution, output_file, !plain_output, " ")
}
