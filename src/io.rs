use std::{
    fs::File,
    io::{self, BufReader, Read, Write},
    path::Path,
};

use anyhow::{Result, anyhow};

use crate::sudoku::Grid;

/// Reads input content into a [Grid].
///
/// If input_file is None, then input will be read from stdin.
///
/// The input content can either be exactly one of:
/// (i)  a 9x9 char grid, with digits 1-9 in the appropriate positions.
/// (ii) a 13x13 char grid, which is the same as the 9x9 grid, but with an additional 1-char border
///      around each 3x3 digit square.
/// Non-digit chars, as well as the digit 0, will be regarded as blanks or part of the grid border.
///
/// Examples of accepted input:
///
/// (a)
/// ```text
/// 53..7....
/// 6..195...
/// .98....6.
/// 8...6...3
/// 4..8.3..1
/// 7...2...6
/// .6....28.
/// ...419..5
/// ....8..79
/// ```
///
/// (b)
/// ```text
/// +---+---+---+
/// |53 | 7 |   |
/// |6  |195|   |
/// | 98|   | 6 |
/// +---+---+---+
/// |8  | 6 |  3|
/// |4  |8 3|  1|
/// |7  | 2 |  6|
/// +---+---+---+
/// | 6 |   |28 |
/// |   |419|  5|
/// |   | 8 | 79|
/// +---+---+---+
/// ```
pub fn read_to_grid<P: AsRef<Path>>(input_file: Option<P>) -> Result<Grid> {
    let mut buffer = String::new();
    let mut reader: Box<dyn Read> = if let Some(input_file) = input_file {
        Box::new(File::open(input_file)?)
    } else {
        Box::new(BufReader::new(io::stdin().lock()))
    };
    reader.read_to_string(&mut buffer)?;

    let mut lines = buffer
        .lines()
        .filter(|line| !line.is_empty())
        .map(String::from)
        .collect::<Vec<_>>();
    if lines.len() != 9 && lines.len() != 13 {
        return Err(anyhow!("Invalid input: incorrect number of rows."));
    }

    if lines.len() == 13 {
        if !lines.iter().all(|line| line.chars().count() == 13) {
            return Err(anyhow!("Invalid input: incorrect row len."));
        }

        // Extract the embedded digits from the grid.

        let extract_digits = |line: String| -> String {
            line.chars()
                .enumerate()
                .filter_map(|(index, c)| {
                    if index == 0 || index == 4 || index == 8 || index == 12 {
                        None
                    } else {
                        Some(c)
                    }
                })
                .collect()
        };

        lines = lines
            .into_iter()
            .enumerate()
            .filter_map(|(index, line)| {
                if index == 0 || index == 4 || index == 8 || index == 12 {
                    None
                } else {
                    Some(extract_digits(line))
                }
            })
            .collect::<Vec<_>>();
    }

    let mut grid = [[0; 9]; 9];
    for (row, line) in lines.into_iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c.is_ascii_digit()
                && let Some(digit) = c.to_digit(10)
                && digit != 0
            {
                grid[row][col] = digit as u8;
            }
        }
    }

    Ok(grid)
}

/// Writes a [Grid] to output.
///
/// If output_file is None, then output will be written to stdout.
///
/// By default, a 9x9 char grid will be written. If border is true, then the output becomes a 13x13
/// char grid, which is the same 9x9 with an additional 1-char border around each 3x3 digit square.
///
/// Any 0 value in the grid will be replaced by blank_char in the output.
pub fn write_grid<P: AsRef<Path>>(
    grid: Grid,
    output_file: Option<P>,
    border: bool,
    blank_char: &str,
) -> Result<()> {
    let output = if border {
        grid_to_border_string(grid, blank_char)
    } else {
        grid_to_string(grid, blank_char)
    };

    let mut writer: Box<dyn Write> = if let Some(output_file) = output_file {
        Box::new(File::create(output_file)?)
    } else {
        Box::new(io::stdout().lock())
    };
    writer.write_all(output.as_bytes())?;

    Ok(())
}

/// Converts a [Grid] to a String for printing.
///
/// Output will end with a newline char.
fn grid_to_string(grid: Grid, blank_char: &str) -> String {
    let mut output = grid
        .map(|row| String::from_utf8_lossy(&row.map(|byte| byte + b'0')).replace("0", blank_char))
        .join("\n");
    output.push('\n');

    output
}

/// Converts a [Grid] to a String for printing.
///
/// Output will end with a newline char.
fn grid_to_border_string(grid: Grid, blank_char: &str) -> String {
    format!(
        "╔═══╤═══╤═══╗\n{}╟───┼───┼───╢\n{}╟───┼───┼───╢\n{}╚═══╧═══╧═══╝\n",
        &grid[..3]
            .iter()
            .map(|row| grid_row_to_border_string(*row))
            .collect::<String>(),
        &grid[3..6]
            .iter()
            .map(|row| grid_row_to_border_string(*row))
            .collect::<String>(),
        &grid[6..]
            .iter()
            .map(|row| grid_row_to_border_string(*row))
            .collect::<String>(),
    )
    .replace("0", blank_char)
}

fn grid_row_to_border_string(row: [u8; 9]) -> String {
    format!(
        "║{}{}{}│{}{}{}│{}{}{}║\n",
        row[0], row[1], row[2], row[3], row[4], row[5], row[6], row[7], row[8],
    )
}
