package cast

import "testing"

func TestToInt(t *testing.T) {
	tests := []struct {
		name  string
		input interface{}
		want  int
	}{
		{"positive number string", "42", 42},
		{"negative number string", "-7", -7},
		{"zero string", "0", 0},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := ToInt(tt.input); got != tt.want {
				t.Errorf("ToInt() = %v, want %v", got, tt.want)
			}
		})
	}
}

func TestToString(t *testing.T) {
	tests := []struct {
		name  string
		input interface{}
		want  string
	}{
		{"positive int", 42, "42"},
		{"negative int", -7, "-7"},
		{"zero", 0, "0"},
		{"byte", byte('a'), "a"},
		{"rune", 'z', "z"},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := ToString(tt.input); got != tt.want {
				t.Errorf("ToString() = %v, want %v", got, tt.want)
			}
		})
	}
}

func TestToASCIICode(t *testing.T) {
	tests := []struct {
		name  string
		input interface{}
		want  int
	}{
		{"capital A string", "A", 65},
		{"lowercase a string", "a", 97},
		{"byte A", byte('A'), 65},
		{"rune a", 'a', 97},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := ToASCIICode(tt.input); got != tt.want {
				t.Errorf("ToASCIICode() = %v, want %v", got, tt.want)
			}
		})
	}
}

func TestASCIIIntToChar(t *testing.T) {
	tests := []struct {
		name  string
		input int
		want  string
	}{
		{"capital A", 65, "A"},
		{"lowercase a", 97, "a"},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := ASCIIIntToChar(tt.input); got != tt.want {
				t.Errorf("ASCIIIntToChar() = %v, want %v", got, tt.want)
			}
		})
	}
}
