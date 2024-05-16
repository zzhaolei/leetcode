package main

import (
	"fmt"
	"strconv"
)

func addStrings(num1 string, num2 string) string {
	var (
		add int
		ans string
	)
	for i, j := len(num1)-1, len(num2)-1; i >= 0 || j >= 0 || add != 0; i, j = i-1, j-1 {
		var x, y int
		if i >= 0 {
			x = int(num1[i] - '0')
		}
		if j >= 0 {
			y = int(num2[j] - '0')
		}
		s := x + y + add
		ans = strconv.Itoa(s%10) + ans
		add = s / 10
	}
	return ans
}

func main() {
	fmt.Println(addStrings("11", "123"))
}
