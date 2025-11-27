# Advent of Code 2025

Solutions for [Advent of Code 2025](https://adventofcode.com/2025) in Go.

## Project Structure

```
.
├── 2025/                  # Solutions for each day
│   └── dayXX/             # Each day has its own package
│       ├── main.go        # Solution code
│       ├── main_test.go   # Tests (with example inputs)
│       └── input.txt      # Your personal puzzle input
├── cast/                  # Type casting utilities
├── mathy/                 # Math helper functions
├── util/                  # General utilities (file reading, etc.)
├── go.mod                 # Go module file
└── README.md
```

## Getting Started

### Prerequisites

- Go 1.21 or higher

### Running Solutions

Navigate to a day's directory and run:

```bash
# Run part 1
go run . -part 1

# Run part 2
go run . -part 2
```

### Running Tests

```bash
# Run tests for a specific day
cd 2025/day01
go test -v

# Run all tests
go test ./...
```

## Utility Packages

### `cast` - Type Casting
- `ToInt(val)` - Convert string to int
- `ToString(val)` - Convert int, byte, or rune to string
- `ToASCIICode(val)` - Get ASCII code of a character
- `ASCIIIntToChar(code)` - Convert ASCII code to character

### `mathy` - Math Helpers
- `MaxInt(nums...)` - Find maximum of integers
- `MinInt(nums...)` - Find minimum of integers
- `AbsInt(n)` - Absolute value of integer
- `SumIntSlice(nums)` - Sum of integer slice
- `MultiplyIntSlice(nums)` - Product of integer slice
- `ManhattanDistance(x1, y1, x2, y2)` - Manhattan distance
- `PythagoreanDistance(x1, y1, x2, y2)` - Euclidean distance
- `LeastCommonMultiple(a, b)` - LCM of two numbers
- `GreatestCommonDivisor(a, b)` - GCD of two numbers

### `util` - General Utilities
- `ReadFile(path)` - Read file relative to caller
- `Dirname()` - Get directory of calling file
- `SplitLines(input)` - Split string into lines

## Creating a New Day

1. Copy the `2025/day01` directory:
   ```bash
   cp -r 2025/day01 2025/dayXX
   ```

2. Replace your puzzle input in `input.txt`

3. Update the example and expected values in `main_test.go`

4. Implement your solution in `main.go`

## Template Features

- **Embedded Input**: Uses Go's `//go:embed` directive to embed puzzle input
- **Part Selection**: Run part 1 or 2 via command-line flag
- **Testing Structure**: Table-driven tests with example and actual inputs

## License

This is personal project code for educational purposes.
