package main

import (
	"fmt"
	"math"
	"strconv"
)

func countDaysTogether(arriveAlice string, leaveAlice string, arriveBob string, leaveBob string) int {
	months := [12]int{31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31}
	calculate := func(s string) int {
		month, _ := strconv.Atoi(s[:2])
		day, _ := strconv.Atoi(s[3:])
		pre_days := 0
		for i := 0; i < month-1; i++ {
			pre_days += months[i]
		}
		return pre_days + day
	}

	ans := math.Max(0, (math.Min(float64(calculate(leaveAlice)), float64(calculate(leaveBob))) - math.Max(float64(calculate(arriveAlice)), float64(calculate(arriveBob))) + 1))
	return int(ans)
}

func main() {
	fmt.Println(countDaysTogether("08-15", "08-18", "08-16", "08-19"))
	fmt.Println(countDaysTogether("10-01", "10-31", "11-01", "12-31"))
}
