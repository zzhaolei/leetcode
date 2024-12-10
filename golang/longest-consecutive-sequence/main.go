package main

import (
	"fmt"
	"sort"
)

func main() {
	r := longestConsecutive2([]int{0, 3, 7, 2, 5, 8, 4, 6, 0, 1})
	fmt.Println(r, r == 9)

	r = longestConsecutive2([]int{100, 4, 200, 1, 3, 2})
	fmt.Println(r, r == 4)

	r = longestConsecutive2([]int{0})
	fmt.Println(r, r == 1)

	r = longestConsecutive2([]int{})
	fmt.Println(r, r == 0)

	r = longestConsecutive2([]int{1, 2, 0, 1})
	fmt.Println(r, r == 3)

	r = longestConsecutive2([]int{0, 0, 0})
	fmt.Println(r, r == 1)
}

func longestConsecutive(nums []int) int {
	if len(nums) == 0 {
		return 0
	}
	sort.Ints(nums)

	var (
		ans   int
		start int
	)
	for i := 1; i < len(nums); i++ {
		if nums[i] == nums[i-1] {
			start += 1
		} else if nums[i]-1 != nums[i-1] {
			start = i
		} else {
			ans = max(ans, i-start)
		}
	}
	return ans + 1
}

func longestConsecutive2(nums []int) int {
	set := make(map[int]bool, len(nums))
	for _, v := range nums {
		set[v] = true
	}

	var ans int
	for v := range set {
		if set[v-1] {
			continue
		}

		count := 1
		for num := v; set[num+1]; num++ {
			count += 1
		}
		ans = max(ans, count)
	}
	return ans
}
