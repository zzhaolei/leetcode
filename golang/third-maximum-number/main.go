// https://leetcode-cn.com/problems/third-maximum-number/submissions/

package main

import (
	"fmt"
	"math"
)

func thirdMax(nums []int) int {
	var (
		num = int(math.Pow(-2, 31))
		max = num
		mid = num
		min = num
		set = make(map[int]struct{})
	)
	for _, v := range nums {
		if v > max {
			max, mid, min = v, max, mid
		} else if v < max && v > mid {
			mid, min = v, mid
		} else if v < mid && v > min {
			min = v
		}
		set[v] = struct{}{}
	}
	if len(set) >= 3 {
		return min
	} else {
		return max
	}
}

func main() {
	fmt.Println(thirdMax([]int{3, 2, 1}), 1)
	fmt.Println(thirdMax([]int{1, 2}), 2)
	fmt.Println(thirdMax([]int{2, 2, 3, 1}), 1)
	fmt.Println(thirdMax([]int{1, 2, -2147483648}), -2147483648)
	fmt.Println(thirdMax([]int{1, -2147483648, 2}), -2147483648)
	fmt.Println(thirdMax([]int{5, 2, 4, 1, 3, 6, 0}), 4)
}
