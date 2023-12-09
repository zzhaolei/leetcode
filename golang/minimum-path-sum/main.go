package main

import "fmt"

func minPathSum(grid [][]int) int {
	m := len(grid)    // i
	n := len(grid[0]) // j
	dp := make([][]int, m)
	for i := range dp {
		dp[i] = make([]int, n)
	}

	dp[0][0] = grid[0][0]

	// 第一行
	for j := 1; j < n; j++ {
		dp[0][j] = dp[0][j-1] + grid[0][j]
	}
	// 第一列
	for i := 1; i < m; i++ {
		dp[i][0] = dp[i-1][0] + grid[i][0]
	}

	// 其他的值
	for i := 1; i < m; i++ {
		for j := 1; j < n; j++ {
			dp[i][j] = min(dp[i-1][j]+grid[i][j], dp[i][j-1]+grid[i][j])
		}
	}

	return dp[m-1][n-1]
}

func main() {
	var r int
	r = minPathSum([][]int{{1, 3, 1}, {1, 5, 1}, {4, 2, 1}})
	fmt.Println(r)
	r = minPathSum([][]int{{1, 2, 3}, {4, 5, 6}})
	fmt.Println(r)
}
