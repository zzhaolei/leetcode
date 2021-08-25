// https://leetcode-cn.com/problems/median-of-two-sorted-arrays/
package main

import (
	"fmt"
	"sort"
)

func impl1(nums1, nums2 []int) float64 {
	// 合并两个数组，并排序
	lenght := len(nums1) + len(nums2)
	newSlice := make([]int, 0, lenght)
	newSlice = append(newSlice, nums1...)
	newSlice = append(newSlice, nums2...)
	sort.Slice(newSlice, func(i, j int) bool {
		return newSlice[i] < newSlice[j]
	})

	// 取出中间数
	if lenght%2 == 0 {
		return (float64(newSlice[lenght/2]) + float64(newSlice[lenght/2-1])) / 2
	} else {
		return float64(newSlice[lenght/2])
	}
}

func findMedianSortedArrays(nums1 []int, nums2 []int) float64 {
	return impl1(nums1, nums2)
}

func main() {
	fmt.Println(findMedianSortedArrays([]int{1, 3}, []int{2}), 2)
	fmt.Println(findMedianSortedArrays([]int{1, 2}, []int{3, 4}), 2.5)
	fmt.Println(findMedianSortedArrays([]int{0, 0}, []int{0, 0}), 0)
	fmt.Println(findMedianSortedArrays([]int{}, []int{1}), 1)
	fmt.Println(findMedianSortedArrays([]int{2}, []int{}), 2)
}
