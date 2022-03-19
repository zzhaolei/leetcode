package main

import (
	"fmt"
	"sort"
)

func merge(intervals [][]int) [][]int {
	sort.Slice(intervals, func(i, j int) bool {
		return intervals[i][0] < intervals[j][0]
	})
	newSlice := make([][]int, 0, len(intervals))
	length := 0
	for _, i := range intervals {
		length = len(newSlice)
		if length == 0 || newSlice[length-1][1] < i[0] {
			newSlice = append(newSlice, i)
		} else if newSlice[length-1][1] < i[1] {
			newSlice[length-1][1] = i[1]
		}
	}
	return newSlice
}

func main() {
	fmt.Println(merge([][]int{{1, 3}, {2, 6}, {8, 10}, {15, 18}}))
	fmt.Println(merge([][]int{{1, 4}, {0, 4}}))
}
