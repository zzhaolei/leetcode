package main

import "fmt"

func countTime(time string) int {
	ans := 1
	if time[:2] == "??" {
		ans = 24
	} else if time[0] == '?' {
		if time[1] > '3' {
			ans = 2
		} else {
			ans = 3
		}
	} else if time[1] == '?' {
		if time[0] == '2' {
			ans = 4
		} else {
			ans = 10
		}
	}

	if time[3:] == "??" {
		ans *= 60
	} else if time[3] == '?' {
		ans *= 6
	} else if time[4] == '?' {
		ans *= 10
	}
	return ans
}

func main() {
	fmt.Println(countTime("??:??"))
	fmt.Println(countTime("?5:00"))
	fmt.Println(countTime("0?:0?"))
	fmt.Println(countTime("2?:??"))
}
