package main

import "fmt"

func applyOperations(nums []int) []int {
	for i := 0; i < len(nums)-1; i++ {
		if nums[i] == nums[i+1] {
			nums[i], nums[i+1] = nums[i]*2, 0
		}
	}

	for i, j := 0, 1; j < len(nums); j++ {
		if nums[i] == 0 {
			if nums[j] == 0 {
				continue
			}
			nums[i], nums[j] = nums[j], nums[i]
		}
		i++
	}
	return nums
}

func main() {
	fmt.Println(applyOperations([]int{300, 126, 0, 0, 523, 523}))
	fmt.Println(applyOperations([]int{1, 2, 2, 1, 1, 0}))
	fmt.Println(applyOperations([]int{0, 1}))
	fmt.Println(applyOperations([]int{1, 0, 2, 0, 0, 1}))
}
