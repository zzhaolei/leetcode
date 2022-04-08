package main

import "fmt"

func search(nums []int, target int) int {
	count := 0
	for _, i := range nums {
		if i == target {
			count++
		}
	}
	return count
}

func main() {
	fmt.Println(search([]int{5, 7, 7, 8, 8, 10}, 8))
	fmt.Println(search([]int{5, 7, 7, 8, 8, 10}, 6))
}
