// https://leetcode-cn.com/problems/search-insert-position/
package main

import (
	"fmt"
	"sort"
)

func impl1(nums []int, target int) int {
	var (
		left  int
		right int
		mid   int
		value int
	)

	right = len(nums) - 1
	for left <= right {
		mid = left + ((right - left) / 2)
		value = nums[mid]
		if target == value {
			return mid
		}
		if target < value {
			right = mid - 1
		} else {
			left = mid + 1
		}
	}
	return left
}

func impl2(nums []int, target int) int {
	return sort.SearchInts(nums, target)
}

func searchInsert(nums []int, target int) int {
	// return impl1(nums, target)
	return impl2(nums, target)
}

func main() {
	fmt.Println(searchInsert([]int{1, 3, 5, 6}, 5), 2)
	fmt.Println(searchInsert([]int{1, 3, 5, 6}, 2), 1)
	fmt.Println(searchInsert([]int{1, 3, 5, 6}, 7), 4)
	fmt.Println(searchInsert([]int{1, 3, 5, 6}, 0), 0)
}
