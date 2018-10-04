package clock

import "fmt"

const testVersion = 4

type Clock struct {
	hour   int
	minute int
}

func New(hour, minute int) Clock {
	if minute >= 60 {
		n := minute / 60
		hour += n
		minute -= n * 60
	}

	if minute < 0 {
		n := (-minute-1)/60 + 1
		hour -= n
		minute += n * 60
	}

	if hour < 0 {
		n := (-hour-1)/24 + 1
		hour += n * 24
	}

	return Clock{hour: hour % 24, minute: minute}
}

func (c Clock) String() string {
	return fmt.Sprintf("%.2d:%.2d", c.hour, c.minute)
}

func (c Clock) Add(minutes int) Clock {
	return New(c.hour, c.minute+minutes)
}
