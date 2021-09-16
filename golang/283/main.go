// https://leetcode-cn.com/problems/move-zeroes/
package main

import "fmt"

func moveZeroes(nums []int) {
	var (
		zeroCount int
		numsLen   int
	)
	numsLen = len(nums)

	for i, v := range nums {
		if v == 0 {
			zeroCount += 1
		} else {
			nums[i-zeroCount] = v
		}
	}
	for zeroCount > 0 {
		nums[numsLen-zeroCount] = 0
		zeroCount -= 1
	}
}

func main() {
	var nums []int

	nums = []int{0, 1, 0, 3, 12}
	moveZeroes(nums)
	fmt.Println(nums)

	nums = []int{0, 0, 0, 0, 1}
	moveZeroes(nums)
	fmt.Println(nums)
}
