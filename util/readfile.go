// Package util provides utility functions for Advent of Code solutions
package util

import (
	"os"
	"path"
	"path/filepath"
	"runtime"
	"strings"
)

// ReadFile reads a file relative to the caller's directory and returns its contents as a string.
// It trims trailing newlines from the content.
func ReadFile(pathFromCaller string) string {
	// Get the directory of the calling file
	_, filename, _, ok := runtime.Caller(1)
	if !ok {
		panic("Could not find Caller of util.ReadFile")
	}

	// Construct the absolute path
	absolutePath := path.Join(path.Dir(filename), pathFromCaller)

	// Read the file content
	content, err := os.ReadFile(absolutePath)
	if err != nil {
		panic(err)
	}

	// Trim trailing newlines and return
	return strings.TrimRight(string(content), "\n")
}

// Dirname returns the directory of the calling file.
// This is similar to __dirname in Node.js.
func Dirname() string {
	_, filename, _, ok := runtime.Caller(1)
	if !ok {
		panic("getting calling function")
	}
	return filepath.Dir(filename)
}

// SplitLines splits a string into lines
func SplitLines(input string) []string {
	return strings.Split(input, "\n")
}
