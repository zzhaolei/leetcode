package main

import (
	"fmt"
	"sort"
)

func insert(intervals [][]int, newInterval []int) [][]int {
	intervals = append(intervals, newInterval)
	sort.Slice(intervals, func(i, j int) bool { return intervals[i][0] < intervals[j][0] })

	var result [][]int
	for _, v := range intervals {
		value := v
		if result == nil || value[0] > result[len(result)-1][1] {
			result = append(result, value)
		} else if value[1] > result[len(result)-1][1] {
			result[len(result)-1] = []int{result[len(result)-1][0], value[1]}
		}
	}
	return result
}

func main() {
	fmt.Println(insert([][]int{{1, 3}, {6, 9}}, []int{2, 5}))
}
