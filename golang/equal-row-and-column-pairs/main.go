package main

import "fmt"

func equalPairs(grid [][]int) int {
	ans := 0
	for _, iv := range grid {
		for j := 0; j < len(grid); j++ {
			find := true
			for k := 0; k < len(iv); k++ {
				if grid[k][j] != iv[k] {
					find = false
					break
				}
			}
			if find {
				ans += 1
			}
		}
	}
	return ans
}

func main() {
	fmt.Println(equalPairs([][]int{{3, 2, 1}, {1, 7, 6}, {2, 7, 7}}))
	fmt.Println(equalPairs([][]int{{3, 1, 2, 2}, {1, 4, 4, 5}, {2, 4, 2, 2}, {2, 4, 2, 2}}))
}
