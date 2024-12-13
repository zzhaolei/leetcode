package main

import "fmt"

func main() {
	r := getFinalState([]int{2, 1, 3, 5, 6}, 5, 2)
	fmt.Println(r)

	r = getFinalState([]int{1}, 5, 2)
	fmt.Println(r)
}

func getFinalState(nums []int, k int, multiplier int) []int {
	for range k {
		target := 0
		for i, v := range nums[1:] {
			if nums[target] > v {
				target = i + 1
			}
		}
		nums[target] *= multiplier
	}
	return nums
}
