package main

import (
	"fmt"
	"sort"
)

func buyChoco(prices []int, money int) int {
	if len(prices) < 2 {
		return money
	}
	sort.Ints(prices)
	if tmp := money - prices[0] - prices[1]; tmp >= 0 {
		return tmp
	}
	return money
}

func main() {
	fmt.Println(buyChoco([]int{1, 2, 2}, 3))
	fmt.Println(buyChoco([]int{3, 2, 3}, 3))
	fmt.Println(buyChoco([]int{25, 24}, 92))
}
