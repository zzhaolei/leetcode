// https://leetcode-cn.com/problems/running-sum-of-1d-array/
package main

import "fmt"

func runningSum(nums []int) []int {
	var (
		s    []int
		i    int
		j    int
		temp int
	)
	s = make([]int, len(nums))
	for i = 0; i < len(nums); i++ {
		temp = 0
		for j = 0; j <= i; j++ {
			temp += nums[j]
		}
		s[i] = temp
	}
	return s
}

func main() {
	fmt.Println(runningSum([]int{1, 2, 3, 4}), []int{1, 3, 6, 10})
	fmt.Println(runningSum([]int{1, 1, 1, 1, 1}), []int{1, 2, 3, 4, 5})
	fmt.Println(runningSum([]int{3, 1, 2, 10, 1}), []int{3, 4, 6, 16, 17})
}
