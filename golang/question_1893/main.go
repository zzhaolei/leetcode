// https://leetcode-cn.com/problems/check-if-all-the-integers-in-a-range-are-covered/
package main

import (
	"fmt"
	"sort"
)

func isCovered(ranges [][]int, left int, right int) bool {
	if ranges == nil || left > right {
		return false
	}

	sort.Slice(ranges, func(i, j int) bool {
		return ranges[i][0] < ranges[j][0]
	})
	fmt.Println(ranges)
	for _, v := range ranges {
		// 如果 left 被 v 覆盖，那么下一次 left 从 v[1] 开始
		if left >= v[0] && left <= v[1] {
			left = v[1] + 1
		}
	}
	return left > right
}

func main() {
	fmt.Println(isCovered([][]int{{1, 2}, {3, 4}, {5, 6}}, 2, 5))
	fmt.Println(isCovered([][]int{{1, 1}}, 1, 50))
	fmt.Println(isCovered([][]int{{50, 50}}, 1, 50))
	fmt.Println(isCovered([][]int{{37, 49}, {5, 17}, {8, 32}}, 29, 49))
}
