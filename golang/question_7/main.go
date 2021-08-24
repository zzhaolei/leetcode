// https://leetcode-cn.com/problems/reverse-integer/
package main

import (
	"fmt"
	"math"
)

func reverse(x int) int {
	var r int = 0
	for x != 0 {
		mod := x % 10
		x = x / 10
		r = r*10 + mod
	}
	if int(math.Pow(-2, 31)) < r && r < int(math.Pow(2, 31))-1 {
		return r
	}
	return 0
}

func main() {
	fmt.Println(reverse(321))
	fmt.Println(reverse(-321))
	fmt.Println(reverse(120))
	fmt.Println(reverse(0))
}
