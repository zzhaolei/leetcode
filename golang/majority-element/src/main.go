// https://leetcode-cn.com/problems/majority-element/
package main

import (
	"fmt"
	"sort"
)

func impl1(nums []int) int {
	sort.Slice(nums, func(i, j int) bool {
		return nums[i] < nums[j]
	})
	return nums[len(nums)/2]
}

func impl2(nums []int) int {
	for i := 0; i < len(nums)-1; i++ {
		for j := 0; j < len(nums)-i-1; j++ {
			if nums[j] > nums[j+1] {
				nums[j], nums[j+1] = nums[j+1], nums[j]
			}
		}
	}
	return nums[len(nums)/2]
}

func majorityElement(nums []int) int {
	return impl2(nums)
}

func main() {
	fmt.Println(majorityElement([]int{1, 3, 1, 3, 1, 3, 1, 3, 1}), 1)
	fmt.Println(majorityElement([]int{3, 2, 3}), 3)
	fmt.Println(majorityElement([]int{2, 2, 1, 1, 1, 2, 2}), 2)
}
