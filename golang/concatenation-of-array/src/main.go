// https://leetcode-cn.com/problems/concatenation-of-array/
package main

import "fmt"

func impl1(nums []int) []int {
	s := make([]int, 0, len(nums)*2)
	s = append(s, nums...)
	s = append(s, nums...)
	return s
}

func impl2(nums []int) []int {
	s := make([]int, len(nums)*2)
	n := 0
	for i := 0; i < 2; i++ {
		for j := 0; j < len(nums); j++ {
			s[n] = nums[j]
			n += 1
		}
	}
	return s
}

func getConcatenation(nums []int) []int {
	//return impl1(nums)
	return impl2(nums)
}

func main() {
	fmt.Println(getConcatenation([]int{1, 2, 1}), []int{1, 2, 1, 1, 2, 1})
	fmt.Println(getConcatenation([]int{1, 3, 2, 1}), []int{1, 3, 2, 1, 1, 3, 2, 1})
}
