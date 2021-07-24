// https://leetcode-cn.com/problems/rotate-array/comments/
package main

import "fmt"

// impl1 循环移动元素
// 这种方式太慢
func impl1(nums []int, k int) {
	new := nums
	lenght := len(new)
	for i := 0; i < k; i++ {
		new = append(new[lenght-1:], new[:lenght-1]...)
	}
	copy(nums, new)
}

// impl2 采用取余的方式，可以计算指定位置的元素
func impl2(nums []int, k int) {
	lenght := len(nums)
	new := make([]int, lenght)
	for i, v := range nums {
		new[(i+k)%lenght] = v
	}
	copy(nums, new)
}

func rotate(nums []int, k int) {
	// impl1(nums, k)
	impl2(nums, k)
}

func main() {
	nums := []int{1, 2, 3, 4, 5, 6, 7}
	rotate(nums, 3)
	fmt.Println(nums)
}
