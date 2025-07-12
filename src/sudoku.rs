use std::array;

/// 9x9 Sudoku grid in reading order.
///
/// Use 1-9 to represent a digit, and 0 to represent a blank or unknown.
pub type Grid = [[u8; 9]; 9];
pub type Puzzle = Grid;
pub type Solution = Grid;

const DIGITS: [u8; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
const INDICES: [usize; 9] = [0, 1, 2, 3, 4, 5, 6, 7, 8];
const BLANKS_TO_GENERATE: usize = 35;

/// Finds all solutions to the given puzzle, if any.
pub fn solve(puzzle: Puzzle) -> Vec<Solution> {
    if !is_valid_puzzle(puzzle) {
        return vec![];
    }

    let blanks = blanks(puzzle);
    if blanks.is_empty() {
        return vec![puzzle];
    }

    let mut solutions = Vec::new();
    find_solutions(puzzle, 0, &blanks, &DIGITS, &mut solutions);
    solutions
}

/// Finds a solution to the given puzzle, if any.
pub fn solve_any(puzzle: Puzzle) -> Option<Solution> {
    if !is_valid_puzzle(puzzle) {
        return None;
    }

    let blanks = blanks(puzzle);
    if blanks.is_empty() {
        return Some(puzzle);
    }

    find_solution(puzzle, 0, &blanks, &DIGITS)
}

/// Generates a puzzle with an unique solution. The puzzle will be generally considered as
/// easy-to-medium difficulty.
pub fn generate() -> Puzzle {
    let puzzle = create_random_blank_positions(create_random_solution(), BLANKS_TO_GENERATE);
    let puzzle = create_random_blank_row(puzzle).unwrap_or(puzzle);
    create_random_blank_col(puzzle).unwrap_or(puzzle)
}

/// Verifies whether a puzzle has exactly one solution.
fn has_unique_solution(puzzle: Puzzle) -> bool {
    if !is_valid_puzzle(puzzle) {
        return false;
    }

    let blanks = blanks(puzzle);
    if blanks.is_empty() {
        return true;
    }

    let mut count_cache = 0;
    count_solutions(puzzle, 0, &blanks, &DIGITS, &mut count_cache);
    count_cache == 1
}

/// Verifies whether a puzzle is valid -- all digits are in legal positions.
fn is_valid_puzzle(puzzle: Puzzle) -> bool {
    (0..9).all(|index| {
        slice_has_unique_digits(horizontal_slice(puzzle, index))
            && slice_has_unique_digits(vertical_slice(puzzle, index))
            && slice_has_unique_digits(square_slice(puzzle, index))
    })
}

/// Verifies whether a slice has all unique digits, except 0, which is ignored.
fn slice_has_unique_digits(slice: [u8; 9]) -> bool {
    let mut unique_digits = [false; 9];

    for digit in slice {
        if digit == 0 {
            continue;
        }

        let index = (digit - 1) as usize;
        if unique_digits[index] {
            return false;
        }

        unique_digits[index] = true;
    }

    true
}

/// (row, col)
type GridPos = (usize, usize);

/// Finds all the blank positions in a [Puzzle] that need to be filled in to form a [Solution].
fn blanks(puzzle: Puzzle) -> Vec<GridPos> {
    puzzle
        .iter()
        .enumerate()
        .flat_map(|(row, digits)| {
            digits.iter().enumerate().filter_map(
                move |(col, digit)| {
                    if *digit == 0 { Some((row, col)) } else { None }
                },
            )
        })
        .collect()
}

/// Gets a view of a row in a [Puzzle].
fn horizontal_slice(puzzle: Puzzle, row: usize) -> [u8; 9] {
    if !(0..9).contains(&row) {
        panic!("Invalid row index: {row}");
    }

    puzzle[row]
}

/// Gets a view of a col in a [Puzzle].
fn vertical_slice(puzzle: Puzzle, col: usize) -> [u8; 9] {
    if !(0..9).contains(&col) {
        panic!("Invalid col index: {col}");
    }

    array::from_fn(|row| puzzle[row][col])
}

/// Gets a view of a square in a [Puzzle].
///
/// Squares are indexed as follows:
///
/// ```text
/// +-+-+-+
/// |0|1|2|
/// +-+-+-+
/// |3|4|5|
/// +-+-+-+
/// |6|7|8|
/// +-+-+-+
/// ```
///
/// Elements in each square are indexed as follows:
/// ```text
/// +---+
/// |012|
/// |345|
/// |678|
/// +---+
/// ```
fn square_slice(puzzle: Puzzle, square: usize) -> [u8; 9] {
    match square {
        0 => [
            puzzle[0][0],
            puzzle[0][1],
            puzzle[0][2],
            puzzle[1][0],
            puzzle[1][1],
            puzzle[1][2],
            puzzle[2][0],
            puzzle[2][1],
            puzzle[2][2],
        ],
        1 => [
            puzzle[0][3],
            puzzle[0][4],
            puzzle[0][5],
            puzzle[1][3],
            puzzle[1][4],
            puzzle[1][5],
            puzzle[2][3],
            puzzle[2][4],
            puzzle[2][5],
        ],
        2 => [
            puzzle[0][6],
            puzzle[0][7],
            puzzle[0][8],
            puzzle[1][6],
            puzzle[1][7],
            puzzle[1][8],
            puzzle[2][6],
            puzzle[2][7],
            puzzle[2][8],
        ],
        3 => [
            puzzle[3][0],
            puzzle[3][1],
            puzzle[3][2],
            puzzle[4][0],
            puzzle[4][1],
            puzzle[4][2],
            puzzle[5][0],
            puzzle[5][1],
            puzzle[5][2],
        ],
        4 => [
            puzzle[3][3],
            puzzle[3][4],
            puzzle[3][5],
            puzzle[4][3],
            puzzle[4][4],
            puzzle[4][5],
            puzzle[5][3],
            puzzle[5][4],
            puzzle[5][5],
        ],
        5 => [
            puzzle[3][6],
            puzzle[3][7],
            puzzle[3][8],
            puzzle[4][6],
            puzzle[4][7],
            puzzle[4][8],
            puzzle[5][6],
            puzzle[5][7],
            puzzle[5][8],
        ],
        6 => [
            puzzle[6][0],
            puzzle[6][1],
            puzzle[6][2],
            puzzle[7][0],
            puzzle[7][1],
            puzzle[7][2],
            puzzle[8][0],
            puzzle[8][1],
            puzzle[8][2],
        ],
        7 => [
            puzzle[6][3],
            puzzle[6][4],
            puzzle[6][5],
            puzzle[7][3],
            puzzle[7][4],
            puzzle[7][5],
            puzzle[8][3],
            puzzle[8][4],
            puzzle[8][5],
        ],
        8 => [
            puzzle[6][6],
            puzzle[6][7],
            puzzle[6][8],
            puzzle[7][6],
            puzzle[7][7],
            puzzle[7][8],
            puzzle[8][6],
            puzzle[8][7],
            puzzle[8][8],
        ],
        _ => panic!("Invalid square index: {square}"),
    }
}

/// Finds a [Solution] to a [Puzzle] by backtracking.
///
/// blank_index is the index into blanks, representing the next blank space to fill.
///
/// digits is the sequence of digits to use for searching. For all practical purposes, digits should
/// contain all of 1..=9.
fn find_solution(
    mut puzzle: Puzzle,
    blank_index: usize,
    blanks: &[GridPos],
    digits: &[u8; 9],
) -> Option<Solution> {
    if blank_index == blanks.len() {
        // We have run out of blanks to fill, so this is a solution.
        return Some(puzzle);
    }

    let (row, col) = blanks[blank_index];

    for digit in digits {
        puzzle[row][col] = *digit;

        if !is_valid_puzzle(puzzle) {
            continue;
        }

        if let Some(solution) = find_solution(puzzle, blank_index + 1, blanks, digits) {
            return Some(solution);
        }
    }

    None
}

/// Finds all [Solution]s to a [Puzzle].
///
/// Returns the solutions in the variable. If no solution is found, the Vec will be empty.
///
/// blank_index is the index into blanks, representing the next blank space to fill.
///
/// digits is the sequence of digits to use for searching. For all practical purposes, digits should
/// contain all of 1..=9.
fn find_solutions(
    mut puzzle: Puzzle,
    blank_index: usize,
    blanks: &[GridPos],
    digits: &[u8; 9],
    solutions: &mut Vec<Solution>,
) {
    if blank_index == blanks.len() {
        // We have run out of blanks to fill, so this is a solution.
        solutions.push(puzzle);

        return;
    }

    let (row, col) = blanks[blank_index];

    for digit in digits {
        puzzle[row][col] = *digit;

        if !is_valid_puzzle(puzzle) {
            continue;
        }

        find_solutions(puzzle, blank_index + 1, blanks, digits, solutions);
    }
}

/// Checks whether a [Puzzle] has 0, 1 or 2 or more [Solution]s.
///
/// Returns the number of solutions (0, 1, or 2) in count_cache. If the puzzle has two or more
/// solutions, count_cache will be 2.
///
/// blank_index is the index into blanks, representing the next blank space to fill.
///
/// digits is the sequence of digits to use for searching. For all practical purposes, digits should
/// contain all of 1..=9.
fn count_solutions(
    mut puzzle: Puzzle,
    blank_index: usize,
    blanks: &[GridPos],
    digits: &[u8; 9],
    count_cache: &mut u8,
) {
    if blank_index == blanks.len() {
        // We have run out of blanks to fill, so this is a solution.
        *count_cache += 1;

        return;
    }

    let (row, col) = blanks[blank_index];

    for digit in digits {
        puzzle[row][col] = *digit;

        if !is_valid_puzzle(puzzle) {
            continue;
        }

        count_solutions(puzzle, blank_index + 1, blanks, digits, count_cache);
        if *count_cache > 1 {
            return;
        }
    }
}

/// Creates a random [Solution].
fn create_random_solution() -> Solution {
    let mut digits = DIGITS;
    fastrand::shuffle(&mut digits);

    // Search for a solution for an empty puzzle, but we jumble up the digits to fill.

    let puzzle = [[0; 9]; 9];
    let blanks: Vec<GridPos> = (0..9)
        .flat_map(|row| (0..9).map(move |col| (row, col)))
        .collect();

    loop {
        if let Some(solution) = find_solution(puzzle, 0, &blanks, &digits) {
            return solution;
        }
    }
}

/// Creates random blanks in the given [Puzzle]. Ensures that the resultant [Puzzle] retains its
/// unique [Solution].
fn create_random_blank_positions(puzzle: Puzzle, count: usize) -> Puzzle {
    if !(1..=64).contains(&count) {
        panic!("Invalid count: {count}");
    }

    let mut positions: Vec<GridPos> = (0..9)
        .flat_map(|row| (0..9).map(move |col| (row, col)))
        .collect();
    fastrand::shuffle(&mut positions);

    let mut puzzle = puzzle;

    let mut blanks_created = 0;
    for (row, col) in positions {
        if blanks_created == count {
            break;
        }

        let orig_digit = puzzle[row][col];
        puzzle[row][col] = 0;

        if has_unique_solution(puzzle) {
            blanks_created += 1;
        } else {
            puzzle[row][col] = orig_digit;
        }
    }

    puzzle
}

/// Creates a randomly chosen blank row in the given [Puzzle]. Ensures that the resultant [Puzzle]
/// retains its unique [Solution].
///
/// Returns None if every row fails to retain the unique [Solution] after blanking.
fn create_random_blank_row(puzzle: Puzzle) -> Option<Puzzle> {
    let mut rows = INDICES;
    fastrand::shuffle(&mut rows);

    for row in rows {
        let mut puzzle = puzzle;
        for col in 0..9 {
            puzzle[row][col] = 0;
        }

        if has_unique_solution(puzzle) {
            return Some(puzzle);
        }
    }

    None
}

/// Creates a randomly chosen blank col in the given [Puzzle]. Ensures that the resultant [Puzzle]
/// retains its unique [Solution].
///
/// Returns None if every row fails to retain the unique [Solution] after blanking.
fn create_random_blank_col(puzzle: Puzzle) -> Option<Puzzle> {
    let mut cols = INDICES;
    fastrand::shuffle(&mut cols);

    for col in cols {
        let mut puzzle = puzzle;
        for row in puzzle.iter_mut() {
            row[col] = 0;
        }

        if has_unique_solution(puzzle) {
            return Some(puzzle);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const SLICE_TEST_1: Puzzle = [
        [0, 0, 0, 1, 1, 1, 2, 2, 2],
        [0, 0, 0, 1, 1, 1, 2, 2, 2],
        [0, 0, 0, 1, 1, 1, 2, 2, 2],
        [3, 3, 3, 4, 4, 4, 5, 5, 5],
        [3, 3, 3, 4, 4, 4, 5, 5, 5],
        [3, 3, 3, 4, 4, 4, 5, 5, 5],
        [6, 6, 6, 7, 7, 7, 8, 8, 8],
        [6, 6, 6, 7, 7, 7, 8, 8, 8],
        [6, 6, 6, 7, 7, 7, 8, 8, 8],
    ];
    const SLICE_TEST_2: Puzzle = [
        [1, 2, 3, 1, 2, 3, 1, 2, 3],
        [4, 5, 6, 4, 5, 6, 4, 5, 6],
        [7, 8, 9, 7, 8, 9, 7, 8, 9],
        [1, 2, 3, 1, 2, 3, 1, 2, 3],
        [4, 5, 6, 4, 5, 6, 4, 5, 6],
        [7, 8, 9, 7, 8, 9, 7, 8, 9],
        [1, 2, 3, 1, 2, 3, 1, 2, 3],
        [4, 5, 6, 4, 5, 6, 4, 5, 6],
        [7, 8, 9, 7, 8, 9, 7, 8, 9],
    ];

    #[test]
    fn check_horizontal_slice() {
        assert_eq!(
            horizontal_slice(SLICE_TEST_1, 0),
            [0, 0, 0, 1, 1, 1, 2, 2, 2]
        );
        assert_eq!(
            horizontal_slice(SLICE_TEST_1, 4),
            [3, 3, 3, 4, 4, 4, 5, 5, 5]
        );
        assert_eq!(
            horizontal_slice(SLICE_TEST_1, 8),
            [6, 6, 6, 7, 7, 7, 8, 8, 8]
        );

        assert_eq!(
            horizontal_slice(SLICE_TEST_1, 1),
            horizontal_slice(SLICE_TEST_1, 2)
        );
        assert_eq!(
            horizontal_slice(SLICE_TEST_1, 3),
            horizontal_slice(SLICE_TEST_1, 5)
        );
        assert_eq!(
            horizontal_slice(SLICE_TEST_1, 6),
            horizontal_slice(SLICE_TEST_1, 7)
        );

        for index in [0, 3, 6] {
            assert_eq!(
                horizontal_slice(SLICE_TEST_2, index),
                [1, 2, 3, 1, 2, 3, 1, 2, 3]
            );
        }
        for index in [1, 4, 7] {
            assert_eq!(
                horizontal_slice(SLICE_TEST_2, index),
                [4, 5, 6, 4, 5, 6, 4, 5, 6]
            );
        }
        for index in [2, 5, 8] {
            assert_eq!(
                horizontal_slice(SLICE_TEST_2, index),
                [7, 8, 9, 7, 8, 9, 7, 8, 9]
            );
        }
    }

    #[test]
    #[should_panic]
    fn check_invalid_horizontal_slice() {
        horizontal_slice(SLICE_TEST_1, 9);
    }

    #[test]
    fn check_vertical_slice() {
        assert_eq!(vertical_slice(SLICE_TEST_1, 0), [0, 0, 0, 3, 3, 3, 6, 6, 6]);
        assert_eq!(vertical_slice(SLICE_TEST_1, 4), [1, 1, 1, 4, 4, 4, 7, 7, 7]);
        assert_eq!(vertical_slice(SLICE_TEST_1, 8), [2, 2, 2, 5, 5, 5, 8, 8, 8]);

        assert_eq!(
            vertical_slice(SLICE_TEST_1, 1),
            vertical_slice(SLICE_TEST_1, 2)
        );
        assert_eq!(
            vertical_slice(SLICE_TEST_1, 3),
            vertical_slice(SLICE_TEST_1, 5)
        );
        assert_eq!(
            vertical_slice(SLICE_TEST_1, 6),
            vertical_slice(SLICE_TEST_1, 7)
        );

        for index in [0, 3, 6] {
            assert_eq!(
                vertical_slice(SLICE_TEST_2, index),
                [1, 4, 7, 1, 4, 7, 1, 4, 7]
            );
        }
        for index in [1, 4, 7] {
            assert_eq!(
                vertical_slice(SLICE_TEST_2, index),
                [2, 5, 8, 2, 5, 8, 2, 5, 8]
            );
        }
        for index in [2, 5, 8] {
            assert_eq!(
                vertical_slice(SLICE_TEST_2, index),
                [3, 6, 9, 3, 6, 9, 3, 6, 9]
            );
        }
    }

    #[test]
    #[should_panic]
    fn check_invalid_vertical_slice() {
        vertical_slice(SLICE_TEST_1, 9);
    }

    #[test]
    fn check_square_slice() {
        for index in 0..9 {
            assert!(
                square_slice(SLICE_TEST_1, index)
                    .iter()
                    .all(|digit| *digit == index as u8)
            );
        }

        for index in 0..9 {
            assert_eq!(
                square_slice(SLICE_TEST_2, index),
                [1, 2, 3, 4, 5, 6, 7, 8, 9]
            );
        }
    }

    #[test]
    #[should_panic]
    fn check_invalid_square_slice() {
        square_slice(SLICE_TEST_1, 9);
    }

    #[test]
    fn check_slice_uniqueness() {
        assert!(slice_has_unique_digits([1, 2, 3, 4, 5, 6, 7, 8, 9]));
        assert!(slice_has_unique_digits([9, 8, 7, 6, 5, 4, 3, 2, 1]));

        assert!(slice_has_unique_digits([0, 2, 0, 4, 0, 6, 0, 8, 0]));
        assert!(slice_has_unique_digits([9, 0, 7, 0, 5, 0, 3, 0, 1]));

        assert!(!slice_has_unique_digits([1, 1, 2, 2, 3, 3, 4, 4, 5]));
        assert!(!slice_has_unique_digits([9, 8, 7, 6, 5, 4, 3, 2, 2]));

        assert!(slice_has_unique_digits([0; 9]));
    }

    #[test]
    fn check_random_solution() {
        let solution = create_random_solution();

        assert!(is_valid_puzzle(solution));
        assert!(solution.into_iter().flatten().all(|digit| digit != 0));
    }

    #[test]
    fn check_random_blanks() {
        let solution = create_random_solution();
        assert_eq!(
            solution
                .iter()
                .flatten()
                .filter(|&digit| *digit == 0)
                .count(),
            0
        );

        let puzzle = create_random_blank_positions(solution, BLANKS_TO_GENERATE);
        assert_eq!(
            puzzle.iter().flatten().filter(|&digit| *digit == 0).count(),
            BLANKS_TO_GENERATE
        );
    }
}
