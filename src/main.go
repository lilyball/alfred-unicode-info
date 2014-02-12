package main

import (
	"code.google.com/p/go-uuid/uuid"
	"fmt"
	"github.com/kballard/go-alfred/alfred"
	"os"
	"regexp"
	"strconv"
)

func main() {
	if len(os.Args) != 2 {
		fmt.Fprintf(os.Stderr, "usage: %s text\n", os.Args[0])
		os.Exit(2)
	}

	text := os.Args[1]

	// check if this is of the form U+1234
	if matches := regexp.MustCompile("^\\s*U\\+([[:xdigit:]]{1,8})\\s*$").FindStringSubmatch(text); matches != nil {
		if decodeCodepoint(matches[1]) {
			return
		}
	}
	transformText(text)
}

func transformText(text string) {
	newtext, err := transform(text, "Any-Name", false)
	if err != nil {
		fmt.Fprintln(os.Stderr, "error:", err)
		os.Exit(1)
	}

	chars, err := Parse(text, newtext)
	retcode := 0
	if err != nil {
		fmt.Fprintln(os.Stderr, "error:", err)
		retcode = 1
		if len(chars) == 0 {
			os.Exit(retcode)
		}
	}

	items := make([]alfred.Item, len(chars))
	for i, char := range chars {
		codepoint := fmt.Sprintf("U+%04X", char.Char)
		items[i] = alfred.Item{
			Title:    char.Name,
			Subtitle: codepoint,
			Arg:      codepoint + " " + char.Name,
			Uid:      uuid.New(),
			Valid:    true,
			Icon: alfred.Icon{
				Type:  alfred.IconTypePath,
				Value: "icon.png",
			},
		}
	}

	bytes, err := alfred.Encode(items)
	if err != nil {
		fmt.Fprintln(os.Stderr, "Error encoding XML:", err)
		os.Exit(1)
	}
	if _, err := os.Stdout.Write(bytes); err != nil {
		fmt.Fprintln(os.Stderr, "Error writing XML:", err)
		os.Exit(1)
	}
}

func decodeCodepoint(text string) bool {
	num, err := strconv.ParseInt(text, 16, 64)
	if err != nil {
		panic(err)
	}
	if num == 0 {
		return false
	}
	text = string([]rune{rune(num)})
	newtext, err := transform(text, "Any-Name", false)
	if err != nil {
		fmt.Fprintln(os.Stderr, "error:", err)
		os.Exit(1)
	}

	chars, err := Parse(text, newtext)
	if err != nil {
		fmt.Fprintln(os.Stderr, "error:", err)
		os.Exit(1)
	}
	if len(chars) != 1 {
		fmt.Fprintf(os.Stderr, "error: unexpected number of chars; expected 1, got %d\n", len(chars))
		os.Exit(1)
	}

	char := chars[0]
	codepoint := fmt.Sprintf("U+%04X", char.Char)
	items := []alfred.Item{
		alfred.Item{
			Title:    text,
			Subtitle: codepoint + " " + char.Name,
			Arg:      text,
			Uid:      uuid.New(),
			Valid:    true,
			Icon: alfred.Icon{
				Type:  alfred.IconTypePath,
				Value: "icon.png",
			},
		},
	}

	bytes, err := alfred.Encode(items)
	if err != nil {
		fmt.Fprintln(os.Stderr, "Error encoding XML:", err)
		os.Exit(1)
	}
	if _, err := os.Stdout.Write(bytes); err != nil {
		fmt.Fprintln(os.Stderr, "Error writing XML:", err)
		os.Exit(1)
	}
	return true
}
