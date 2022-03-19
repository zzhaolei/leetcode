// https://leetcode-cn.com/problems/latest-time-by-replacing-hidden-digits/
package main

import "fmt"

func maximumTime(time string) string {
	newSlice := []byte(time)
	if newSlice[0] == '?' && newSlice[1] == '?' && newSlice[3] == '?' && newSlice[4] == '?' {
		return "23:59"
	}

	if newSlice[0] == '?' {
		if newSlice[1] >= '4' && newSlice[1] <= '9' {
			newSlice[0] = '1'
		} else {
			newSlice[0] = '2'
		}
	}
	if newSlice[1] == '?' {
		if newSlice[0] == '2' {
			newSlice[1] = '3'
		} else {
			newSlice[1] = '9'
		}
	}

	if newSlice[3] == '?' {
		newSlice[3] = '5'
	}
	if newSlice[4] == '?' {
		newSlice[4] = '9'
	}
	return string(newSlice)
}

func main() {
	fmt.Println(maximumTime("??:??"))
	fmt.Println(maximumTime("2?:?0"))
	fmt.Println(maximumTime("??:?9"))
	fmt.Println(maximumTime("??:3?"))
}
