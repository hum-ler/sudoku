# sudoku

Generates and solves Sudoku puzzles.

## Usage

```
Usage: sudoku <COMMAND>

Commands:
  gen    Generates a puzzle
  solve  Solves a puzzle
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

Example:
```bash
sudoku gen |tee /dev/tty |sudoku solve
```

### Generate a puzzle (gen command)

```
Usage: sudoku gen [OPTIONS]

Options:
  -o, --output <OUTPUT_FILE>  The output file to write to (overwrites), omit to write to stdout
  -n, --no-border             Do not draw border to format the puzzle
  -b, --blank <BLANK_CHAR>    The character that represents a blank space [default: " "]
  -h, --help                  Print help
```

### Solve a puzzle (solve command)

```
Usage: sudoku solve [OPTIONS]

Options:
  -i, --input <INPUT_FILE>    The input file to read from, omit to read from stdin
  -o, --output <OUTPUT_FILE>  The output file to write to (overwrites), omit to write to stdout
  -n, --no-border             Do not draw border to format the solution
  -h, --help                  Print help
```

## Input format

Refer to [`read_to_puzzle()`](target/doc/sudoku/prelude/fn.read_to_puzzle.html).
