// https://leetcode-cn.com/problems/intersection-of-two-arrays/
package main

import "fmt"

func intersection(nums1 []int, nums2 []int) []int {
	var (
		r []int
		m = make(map[int]struct{})
	)
	for _, v := range nums1 {
		if _, ok := m[v]; !ok {
			m[v] = struct{}{}
		}
	}
	for _, v := range nums2 {
		if _, ok := m[v]; ok {
			r = append(r, v)
			delete(m, v)
		}
	}

	return r
}

func main() {
	fmt.Println(intersection([]int{1, 2, 2, 1}, []int{2, 2}))
	fmt.Println(intersection([]int{4, 9, 5}, []int{9, 4, 9, 8, 4}))
}
