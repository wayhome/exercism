package raindrops

import (
	"strconv"
)

const testVersion = 3

func Convert(number int) (res string) {
	if number%3 == 0 {
		res += "Pling"
	}

	if number%5 == 0 {
		res += "Plang"
	}

	if number%7 == 0 {
		res += "Plong"
	}

	if len(res) == 0 {
		res = strconv.Itoa(number)
	}
	return

}
