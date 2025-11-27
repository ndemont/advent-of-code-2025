package main

import (
	"flag"
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"github.com/ndemont/advent-of-code-2025/util"
)

// Configuration for this day
const (
	year = 2025
	day  = 6
)

var input string

func loadInput() string {
	// Get the current working directory (where the command is run from)
	cwd, err := os.Getwd()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Warning: could not get working directory: %v\n", err)
		cwd, _ = filepath.Abs(".")
	}
	inputPath := filepath.Join(cwd, "input.txt")

	// Get input: try local file first, fetch from AoC if empty/missing
	return strings.TrimRight(util.GetInput(year, day, inputPath), "\n")
}

func main() {
	var part int
	flag.IntVar(&part, "part", 1, "part 1 or 2")
	flag.Parse()
	fmt.Println("Running part", part)

	// Load input when running main (not in tests)
	input = loadInput()

	if len(input) == 0 {
		fmt.Println("No input available. Set AOC_SESSION environment variable to fetch input.")
		return
	}

	if part == 1 {
		ans := part1(input)
		fmt.Println("Output:", ans)
	} else {
		ans := part2(input)
		fmt.Println("Output:", ans)
	}
}

func part1(input string) int {
	// TODO: Implement solution for part 1
	_ = parseInput(input)
	return 0
}

func part2(input string) int {
	// TODO: Implement solution for part 2
	_ = parseInput(input)
	return 0
}

func parseInput(input string) []string {
	lines := strings.Split(input, "\n")
	return lines
}
