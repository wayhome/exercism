package acronym

import (
	"strings"
)

const testVersion = 3

func Abbreviate(input string) string {
	result := ""
	for _, s := range strings.Split(input, " ") {
		for _, s2 := range strings.Split(s, "-") {
			result += strings.ToUpper(string(s2[0]))
		}
	}
	return result
}
