package mathy

import (
	"math"
)

// PythagoreanDistance calculates the Euclidean distance between two points
func PythagoreanDistance(x1, y1, x2, y2 int) float64 {
	xDiff := float64(x1 - x2)
	yDiff := float64(y1 - y2)

	sumOfSquares := math.Pow(xDiff, 2) + math.Pow(yDiff, 2)
	return math.Sqrt(sumOfSquares)
}

// ManhattanDistance calculates the Manhattan distance between two points
func ManhattanDistance(x1, y1, x2, y2 int) int {
	return AbsInt(x1-x2) + AbsInt(y1-y2)
}
