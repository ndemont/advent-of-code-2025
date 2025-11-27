# Advent of Code 2025

Solutions for [Advent of Code 2025](https://adventofcode.com/2025) in Go.

## Project Structure

```
.
├── 2025/                  # Solutions for each day
│   └── dayXX/             # Each day has its own package
│       ├── main.go        # Solution code
│       ├── main_test.go   # Tests (with example inputs)
│       └── input.txt      # Your personal puzzle input (auto-fetched)
├── cast/                  # Type casting utilities
├── mathy/                 # Math helper functions
├── util/                  # General utilities (file reading, input fetching, etc.)
├── go.mod                 # Go module file
└── README.md
```

## Getting Started

### Prerequisites

- Go 1.21 or higher

### Setting Up Session Cookie (for automatic input fetching)

To automatically download your puzzle input from adventofcode.com:

1. Log in to [adventofcode.com](https://adventofcode.com)
2. Open your browser's developer tools (F12)
3. Go to Application/Storage > Cookies > adventofcode.com
4. Copy the value of the `session` cookie
5. Set it as an environment variable:
   ```bash
   export AOC_SESSION="your_session_cookie_here"
   ```

With this set, the template will automatically fetch your puzzle input when you run a solution.

### Running Solutions

Navigate to a day's directory and run:

```bash
# Run part 1
go run . -part 1

# Run part 2
go run . -part 2
```

The input will be automatically fetched from adventofcode.com and cached locally in `input.txt`.

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
- `FetchInput(year, day)` - Fetch puzzle input from adventofcode.com
- `GetInput(year, day, localPath)` - Get input (from cache or fetch from AoC)

## Creating a New Day

1. Copy the `2025/day01` directory:
   ```bash
   cp -r 2025/day01 2025/dayXX
   ```

2. Update the `year` and `day` constants in `main.go`

3. Update the example and expected values in `main_test.go`

4. Run the solution - input will be fetched automatically:
   ```bash
   go run . -part 1
   ```

## Template Features

- **Automatic Input Fetching**: Fetches your puzzle input from adventofcode.com using your session cookie
- **Local Caching**: Input is cached locally in `input.txt` after first fetch
- **Part Selection**: Run part 1 or 2 via command-line flag
- **Testing Structure**: Table-driven tests with example and actual inputs

## License

This is personal project code for educational purposes.
