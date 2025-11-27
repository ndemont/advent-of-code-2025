package main

import (
	_ "embed"
	"flag"
	"fmt"
	"strings"
)

//go:embed input.txt
var input string

func init() {
	// Trim trailing newlines from embedded input
	input = strings.TrimRight(input, "\n")
	if len(input) == 0 {
		panic("empty input.txt file")
	}
}

func main() {
	var part int
	flag.IntVar(&part, "part", 1, "part 1 or 2")
	flag.Parse()
	fmt.Println("Running part", part)

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
