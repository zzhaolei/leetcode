package main

import "fmt"

func removeElement(nums []int, val int) int {
	var i int
	for _, v := range nums {
		if v != val {
			nums[i] = v
			i++
		}
	}
	return i
}

func main() {
	s := []int{3, 2, 2, 3}
	fmt.Println(s[:removeElement(s, 3)])
}
