// Package util provides utility functions for Advent of Code solutions
package util

import (
	"fmt"
	"io"
	"net/http"
	"os"
	"path/filepath"
	"strings"
)

// FetchInput downloads the puzzle input for a given year and day from adventofcode.com.
// It requires the AOC_SESSION environment variable to be set with your session cookie.
// The session cookie can be obtained from your browser after logging into adventofcode.com.
//
// Example:
//
//	export AOC_SESSION="your_session_cookie_here"
//	input := util.FetchInput(2024, 15)
func FetchInput(year, day int) string {
	session := os.Getenv("AOC_SESSION")
	if session == "" {
		panic("AOC_SESSION environment variable not set. Get your session cookie from adventofcode.com after logging in.")
	}

	url := fmt.Sprintf("https://adventofcode.com/%d/day/%d/input", year, day)

	req, err := http.NewRequest("GET", url, nil)
	if err != nil {
		panic(fmt.Sprintf("Failed to create request: %v", err))
	}

	// Set the session cookie
	req.AddCookie(&http.Cookie{
		Name:  "session",
		Value: session,
	})

	// Set a user agent as required by AoC - use repo URL or custom value from env
	userAgent := os.Getenv("AOC_USER_AGENT")
	if userAgent == "" {
		userAgent = "advent-of-code-go-template"
	}
	req.Header.Set("User-Agent", userAgent)

	client := &http.Client{}
	resp, err := client.Do(req)
	if err != nil {
		panic(fmt.Sprintf("Failed to fetch input: %v", err))
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		body, _ := io.ReadAll(resp.Body)
		panic(fmt.Sprintf("Failed to fetch input (status %d): %s", resp.StatusCode, string(body)))
	}

	body, err := io.ReadAll(resp.Body)
	if err != nil {
		panic(fmt.Sprintf("Failed to read response: %v", err))
	}

	return strings.TrimRight(string(body), "\n")
}

// FetchAndSaveInput downloads the puzzle input and saves it to a file.
// This is useful for caching the input locally.
func FetchAndSaveInput(year, day int, outputPath string) string {
	input := FetchInput(year, day)

	// Ensure directory exists
	dir := filepath.Dir(outputPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		panic(fmt.Sprintf("Failed to create directory: %v", err))
	}

	// Write to file
	if err := os.WriteFile(outputPath, []byte(input+"\n"), 0644); err != nil {
		panic(fmt.Sprintf("Failed to write input file: %v", err))
	}

	return input
}

// GetInput tries to read input from a local file first.
// If the file doesn't exist or is empty, it fetches from adventofcode.com and caches it.
func GetInput(year, day int, localPath string) string {
	// Try to read from local file first
	content, err := os.ReadFile(localPath)
	if err == nil && len(strings.TrimSpace(string(content))) > 0 {
		return strings.TrimRight(string(content), "\n")
	}

	// File doesn't exist or is empty, fetch from AoC
	fmt.Printf("Fetching input for %d day %d from adventofcode.com...\n", year, day)
	return FetchAndSaveInput(year, day, localPath)
}
