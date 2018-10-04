package accumulate

const testVersion = 1

func Accumulate(given []string, converter func(string) string) []string {
	for i := 0; i < len(given); i++ {
		given[i] = converter(given[i])
	}
	return given
}
