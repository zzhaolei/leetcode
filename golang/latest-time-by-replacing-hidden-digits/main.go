// https://leetcode-cn.com/problems/latest-time-by-replacing-hidden-digits/
package main

import "fmt"

func maximumTime(time string) string {
	new := []byte(time)
	if new[0] == '?' && new[1] == '?' && new[3] == '?' && new[4] == '?' {
		return "23:59"
	}

	if new[0] == '?' {
		if new[1] >= '4' && new[1] <= '9' {
			new[0] = '1'
		} else {
			new[0] = '2'
		}
	}
	if new[1] == '?' {
		if new[0] == '2' {
			new[1] = '3'
		} else {
			new[1] = '9'
		}
	}

	if new[3] == '?' {
		new[3] = '5'
	}
	if new[4] == '?' {
		new[4] = '9'
	}
	return string(new)
}

func main() {
	fmt.Println(maximumTime("??:??"))
	fmt.Println(maximumTime("2?:?0"))
	fmt.Println(maximumTime("??:?9"))
	fmt.Println(maximumTime("??:3?"))
}
