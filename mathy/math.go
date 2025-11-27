// Package mathy provides common math utilities for Advent of Code solutions
package mathy

// MaxInt returns the maximum of the given integers
func MaxInt(nums ...int) int {
	if len(nums) == 0 {
		panic("MaxInt requires at least one argument")
	}
	maxNum := nums[0]
	for _, v := range nums {
		if v > maxNum {
			maxNum = v
		}
	}
	return maxNum
}

// MinInt returns the minimum of the given integers
func MinInt(nums ...int) int {
	if len(nums) == 0 {
		panic("MinInt requires at least one argument")
	}
	minNum := nums[0]
	for _, v := range nums {
		if v < minNum {
			minNum = v
		}
	}
	return minNum
}

// AbsInt returns the absolute value of an integer
func AbsInt(in int) int {
	if in < 0 {
		return -in
	}
	return in
}

// SumIntSlice returns the sum of all integers in a slice
func SumIntSlice(nums []int) int {
	var sum int
	for _, n := range nums {
		sum += n
	}
	return sum
}

// MultiplyIntSlice returns the product of all integers in a slice
func MultiplyIntSlice(nums []int) int {
	product := 1
	for _, n := range nums {
		product *= n
	}
	return product
}
