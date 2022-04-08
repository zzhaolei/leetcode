// https://leetcode-cn.com/problems/missing-number/
package main

import "fmt"

func missingNumber(nums []int) int {
	m := make(map[int]struct{})
	for _, v := range nums {
		m[v] = struct{}{}
	}
	for i := 0; i <= len(nums); i++ {
		if _, ok := m[i]; !ok {
			return i
		}
	}
	return 0
}

func main() {
	fmt.Println(missingNumber([]int{3, 0, 1}), 2)
	fmt.Println(missingNumber([]int{0, 1}), 2)
	fmt.Println(missingNumber([]int{9, 6, 4, 2, 3, 5, 7, 0, 1}), 8)
	fmt.Println(missingNumber([]int{0}), 1)
}
