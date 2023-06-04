package main

import (
	"fmt"
	"sort"
)

func distinctAverages(nums []int) int {
	sort.Ints(nums)

	ans := make(map[float64]struct{})
	i, j := 0, len(nums)-1
	for ; i < j; i++ {
		value := float64(nums[i]+nums[j]) / 2
		ans[value] = struct{}{}
		j--
	}
	return len(ans)
}

func main() {
	fmt.Println(distinctAverages([]int{4, 1, 4, 0, 3, 5}))
	fmt.Println(distinctAverages([]int{9, 5, 7, 8, 7, 9, 8, 2, 0, 7}))
	fmt.Println(distinctAverages([]int{100, 1}))
}
