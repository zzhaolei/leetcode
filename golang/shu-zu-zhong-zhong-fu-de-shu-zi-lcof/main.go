package main

import "fmt"

// 2 <= len(nums) <= 100000
func findRepeatNumber(nums []int) int {
	s := make([]int, len(nums)-1)
	for _, i := range nums {
		if s[i] == 1 {
			return i
		}
		s[i] = 1
	}
	return 0
}

func main() {
	r := findRepeatNumber([]int{2, 3, 1, 0, 2, 5, 3})
	fmt.Println(r)
}
