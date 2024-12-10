package main

import (
	"fmt"
)

func main() {
	nums := []int{0, 1, 0, 3, 12}
	moveZeroes(nums)
	fmt.Println(nums)

	nums = []int{1, 0, 2, 0, 3, 12}
	moveZeroes2(nums)
	fmt.Println(nums)
}

func moveZeroes(nums []int) {
	for i := 0; i < len(nums)-1; i++ {
		if nums[i] == 0 {
			for j := i + 1; j < len(nums); j++ {
				if nums[j] != 0 {
					nums[i], nums[j] = nums[j], nums[i]
					break
				}
			}
		}
	}
}

func moveZeroes2(nums []int) {
	var zeroCount int
	for i, v := range nums {
		if v == 0 {
			zeroCount += 1
		} else {
			// 1, 0, 0, 2, 0, 3 这种的前面有两个个0，当前 i-2=1，会将 2 放入 1 后面，即：
			// 1, 2, 0, 2, 0, 3
			nums[i-zeroCount] = v
		}
	}

	// 在最后面加 0
	for length := len(nums); zeroCount > 0; zeroCount-- {
		nums[length-zeroCount] = 0
	}
}
