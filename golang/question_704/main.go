// https://leetcode-cn.com/problems/binary-search/
package main

import (
	"fmt"
	"sort"
)

func searchImpl1(nums []int, target int) int {
	left, right := 0, len(nums)-1
	var (
		mid int
	)
	for left <= right {
		mid = left + ((right - left) / 2)
		value := nums[mid]

		if target == value {
			return mid
		}
		if target > value {
			left = mid + 1
		} else {
			right = mid - 1
		}
	}
	return -1
}

// searchImpl2 当元素不存在时，此函数返回待插入的索引
func searchImpl2(nums []int, target int) int {
	return sort.SearchInts(nums, target)
}

func searchImpl3(nums []int, target int) int {
	for i, v := range nums {
		if v == target {
			return i
		}
	}
	return -1
}

func search(nums []int, target int) int {
	return searchImpl1(nums, target)
	// return searchImpl2(nums, target)
	// return searchImpl3(nums, target)
}

func main() {
	fmt.Println(search([]int{-1, 0, 3, 5, 9, 12}, -1) == 0)
	fmt.Println(search([]int{-1, 0, 3, 5, 9, 12}, 0) == 1)
	fmt.Println(search([]int{-1, 0, 3, 5, 9, 12}, 2) == -1)
	fmt.Println(search([]int{-1, 0, 3, 5, 9, 12}, 3) == 2)
	fmt.Println(search([]int{-1, 0, 3, 5, 9, 12}, 5) == 3)
	fmt.Println(search([]int{-1, 0, 3, 5, 9, 12}, 9) == 4)
	fmt.Println(search([]int{-1, 0, 3, 5, 9, 12}, 12) == 5)
	fmt.Println(search([]int{-1, 0, 3, 5, 9, 12}, 15) == -1)
}
