package main

import "fmt"

func removeDuplicates(nums []int) int {
	var i int
	for _, v := range nums {
		if nums[i] != v {
			i++
			nums[i] = v
		}
	}
	return i + 1
}

func main() {
	s := []int{0, 0, 1, 1, 1, 2, 2, 3, 3, 4}
	fmt.Println(s[:removeDuplicates(s)])
}
