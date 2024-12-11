package main

import "fmt"

func main() {
	r := semiOrderedPermutation([]int{2, 4, 1, 3})
	fmt.Println(r)

	r = semiOrderedPermutation([]int{2, 4, 5, 1, 3})
	fmt.Println(r)

	r = semiOrderedPermutation([]int{2, 1, 4, 3})
	fmt.Println(r)
}

func semiOrderedPermutation(nums []int) int {
	if nums[0] == 1 && nums[len(nums)-1] == len(nums) {
		return 0
	}

	var (
		f int
		s int
	)
	for i, v := range nums {
		if v == 1 {
			f = i
		} else if v == len(nums) {
			s = i
		}
	}

	ans := f + len(nums) - 1 - s
	if s < f {
		ans -= 1
	}
	return ans
}
