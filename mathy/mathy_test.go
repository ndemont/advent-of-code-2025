package mathy

import "testing"

func TestMaxInt(t *testing.T) {
	tests := []struct {
		name string
		nums []int
		want int
	}{
		{"single number", []int{5}, 5},
		{"positive numbers", []int{1, 5, 3}, 5},
		{"negative numbers", []int{-1, -5, -3}, -1},
		{"mixed numbers", []int{-10, 0, 10}, 10},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := MaxInt(tt.nums...); got != tt.want {
				t.Errorf("MaxInt() = %v, want %v", got, tt.want)
			}
		})
	}
}

func TestMinInt(t *testing.T) {
	tests := []struct {
		name string
		nums []int
		want int
	}{
		{"single number", []int{5}, 5},
		{"positive numbers", []int{1, 5, 3}, 1},
		{"negative numbers", []int{-1, -5, -3}, -5},
		{"mixed numbers", []int{-10, 0, 10}, -10},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := MinInt(tt.nums...); got != tt.want {
				t.Errorf("MinInt() = %v, want %v", got, tt.want)
			}
		})
	}
}

func TestAbsInt(t *testing.T) {
	tests := []struct {
		name string
		in   int
		want int
	}{
		{"positive", 5, 5},
		{"negative", -5, 5},
		{"zero", 0, 0},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := AbsInt(tt.in); got != tt.want {
				t.Errorf("AbsInt() = %v, want %v", got, tt.want)
			}
		})
	}
}

func TestSumIntSlice(t *testing.T) {
	tests := []struct {
		name string
		nums []int
		want int
	}{
		{"empty slice", []int{}, 0},
		{"single number", []int{5}, 5},
		{"positive numbers", []int{1, 2, 3}, 6},
		{"mixed numbers", []int{-1, 0, 1}, 0},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := SumIntSlice(tt.nums); got != tt.want {
				t.Errorf("SumIntSlice() = %v, want %v", got, tt.want)
			}
		})
	}
}

func TestMultiplyIntSlice(t *testing.T) {
	tests := []struct {
		name string
		nums []int
		want int
	}{
		{"empty slice", []int{}, 1},
		{"single number", []int{5}, 5},
		{"positive numbers", []int{2, 3, 4}, 24},
		{"with zero", []int{1, 0, 3}, 0},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := MultiplyIntSlice(tt.nums); got != tt.want {
				t.Errorf("MultiplyIntSlice() = %v, want %v", got, tt.want)
			}
		})
	}
}

func TestLeastCommonMultiple(t *testing.T) {
	tests := []struct {
		name string
		i, j int
		want int
	}{
		{"4 and 6", 4, 6, 12},
		{"3 and 5", 3, 5, 15},
		{"same numbers", 7, 7, 7},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := LeastCommonMultiple(tt.i, tt.j); got != tt.want {
				t.Errorf("LeastCommonMultiple() = %v, want %v", got, tt.want)
			}
		})
	}
}

func TestGreatestCommonDivisor(t *testing.T) {
	tests := []struct {
		name string
		i, j int
		want int
	}{
		{"8 and 12", 8, 12, 4},
		{"17 and 13", 17, 13, 1},
		{"same numbers", 7, 7, 7},
		{"negative and positive", -8, 12, 4},
		{"both negative", -8, -12, 4},
		{"with zero", 8, 0, 8},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := GreatestCommonDivisor(tt.i, tt.j); got != tt.want {
				t.Errorf("GreatestCommonDivisor() = %v, want %v", got, tt.want)
			}
		})
	}
}

func TestManhattanDistance(t *testing.T) {
	tests := []struct {
		name           string
		x1, y1, x2, y2 int
		want           int
	}{
		{"same point", 0, 0, 0, 0, 0},
		{"positive coords", 1, 1, 4, 5, 7},
		{"negative coords", -1, -1, -4, -5, 7},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := ManhattanDistance(tt.x1, tt.y1, tt.x2, tt.y2); got != tt.want {
				t.Errorf("ManhattanDistance() = %v, want %v", got, tt.want)
			}
		})
	}
}
