// https://leetcode-cn.com/problems/single-number/
package main

import "fmt"

func impl1(nums []int) int {
	m := make(map[int]int)
	for _, v := range nums {
		if vv, ok := m[v]; ok {
			m[v] = vv + 1
		} else {
			m[v] = 1
		}
	}
	for k, v := range m {
		if v != 1 {
			continue
		}
		return k
	}
	return 0
}

func impl2(nums []int) int {
	var find bool
	for i, v := range nums {
		find = false
		for j, vv := range nums {
			if j == i {
				continue
			}
			if v == vv {
				find = true
				break
			}
		}
		if !find {
			return v
		}
	}
	return 0
}

func impl3(nums []int) int {
	for _, v := range nums {
		nums[0] ^= v
	}
	return nums[0]
}

func singleNumber(nums []int) int {
	// return impl1(nums)
	// return impl2(nums)
	return impl3(nums)
}

func main() {
	fmt.Println(singleNumber([]int{2, 2, 1}), 1)
	fmt.Println(singleNumber([]int{4, 1, 2, 1, 2}), 4)
}
