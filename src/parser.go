package main

import (
	"errors"
	"strings"
	"unicode/utf8"
)

type CharInfo struct {
	Char rune
	Name string
}

func Parse(input, transformed string) (chars []CharInfo, err error) {
	chars = make([]CharInfo, 0)

	for input != "" || transformed != "" {
		if input == "" {
			err = errors.New("Unexpected termination of input string")
			return
		}
		if transformed == "" {
			err = errors.New("Unexpected termination of transformed string")
			return
		}
		r, l := utf8.DecodeRuneInString(input)
		if r == utf8.RuneError && l == 1 {
			err = errors.New("Malformed utf8 input")
			return
		}
		input = input[l:]
		if transformed[:3] == "\\N{" {
			transformed = transformed[3:]
			end := strings.IndexRune(transformed, '}')
			if end == -1 {
				err = errors.New("Invalid transformed string")
				return
			}
			name := transformed[:end]
			transformed = transformed[end+1:]
			chars = append(chars, CharInfo{r, name})
		} else {
			err = errors.New("Invalid transformed string")
			return
		}
	}

	return
}
