// Package cast provides type casting utilities for Advent of Code solutions.
// Note: These functions panic on errors for faster AoC development.
package cast

import (
	"fmt"
	"strconv"
)

// ToInt converts a value to an int.
// Supported types: string
func ToInt(arg interface{}) int {
	var val int
	switch v := arg.(type) {
	case string:
		var err error
		val, err = strconv.Atoi(v)
		if err != nil {
			panic("error converting string to int: " + err.Error())
		}
	default:
		panic(fmt.Sprintf("unhandled type for int casting: %T", arg))
	}
	return val
}

// ToString converts a value to a string.
// Supported types: int, byte, rune
func ToString(arg interface{}) string {
	switch v := arg.(type) {
	case int:
		return strconv.Itoa(v)
	case byte:
		return string(rune(v))
	case rune:
		return string(v)
	default:
		panic(fmt.Sprintf("unhandled type for string casting: %T", arg))
	}
}

// ASCII code constants
const (
	ASCIICodeCapA   = int('A') // 65
	ASCIICodeCapZ   = int('Z') // 90
	ASCIICodeLowerA = int('a') // 97
	ASCIICodeLowerZ = int('z') // 122
)

// ToASCIICode returns the ASCII code of a given input.
// Supported types: string (length 1), byte, rune
func ToASCIICode(arg interface{}) int {
	switch v := arg.(type) {
	case string:
		if len(v) != 1 {
			panic("can only convert ASCII code for string of length 1")
		}
		return int(v[0])
	case byte:
		return int(v)
	case rune:
		return int(v)
	default:
		panic(fmt.Sprintf("unhandled type for ASCII code conversion: %T", arg))
	}
}

// ASCIIIntToChar returns a one-character string of the given ASCII code
func ASCIIIntToChar(code int) string {
	return string(rune(code))
}
