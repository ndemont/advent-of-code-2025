package mathy

// LeastCommonMultiple returns the LCM of two integers
// Uses (i / gcd) * j to avoid potential integer overflow
func LeastCommonMultiple(i, j int) int {
	gcd := GreatestCommonDivisor(i, j)
	if gcd == 0 {
		return 0
	}
	return (i / gcd) * j
}

// GreatestCommonDivisor returns the GCD of two integers using the Euclidean algorithm
// Works correctly with negative numbers by using absolute values
func GreatestCommonDivisor(i, j int) int {
	// Use absolute values to handle negative inputs
	i = AbsInt(i)
	j = AbsInt(j)

	if j > i {
		i, j = j, i
	}
	if j == 0 {
		return i
	}
	return GreatestCommonDivisor(j, i%j)
}

// LCMSlice returns the LCM of a slice of integers
func LCMSlice(nums []int) int {
	if len(nums) == 0 {
		return 0
	}
	result := nums[0]
	for i := 1; i < len(nums); i++ {
		result = LeastCommonMultiple(result, nums[i])
	}
	return result
}

// GCDSlice returns the GCD of a slice of integers
func GCDSlice(nums []int) int {
	if len(nums) == 0 {
		return 0
	}
	result := nums[0]
	for i := 1; i < len(nums); i++ {
		result = GreatestCommonDivisor(result, nums[i])
	}
	return result
}
