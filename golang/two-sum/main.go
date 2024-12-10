package main

import "fmt"

func main() {
	r := twoSum([]int{2, 7, 11, 15}, 9)
	fmt.Println(r)
}

func twoSum(nums []int, target int) []int {
	for i := 0; i < len(nums); i++ {
		for j := i + 1; j < len(nums); j++ {
			if nums[i]+nums[j] == target {
				return []int{i, j}
			}
		}
	}
	return nil
}
