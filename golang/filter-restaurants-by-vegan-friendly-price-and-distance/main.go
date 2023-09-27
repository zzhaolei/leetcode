package main

import (
	"fmt"
	"sort"
)

func filterRestaurants(restaurants [][]int, veganFriendly int, maxPrice int, maxDistance int) []int {
	result := make([][]int, 0, len(restaurants))
	for _, v := range restaurants {
		if (veganFriendly == 0 || v[2] == veganFriendly) && v[3] <= maxPrice && v[4] <= maxDistance {
			result = append(result, []int{v[0], v[1]})
		}
	}
	sort.SliceStable(result, func(i, j int) bool {
		if result[i][1] == result[j][1] {
			return result[i][0] > result[j][0]
		}
		return result[i][1] > result[j][1]
	})

	n := make([]int, 0, len(result))
	for _, v := range result {
		n = append(n, v[0])
	}
	return n
}

func main() {
	r := filterRestaurants([][]int{{1, 4, 1, 40, 10}, {2, 8, 0, 50, 5}, {3, 8, 1, 30, 4}, {4, 10, 0, 10, 3}, {5, 1, 1, 15, 1}}, 1, 50, 10)
	fmt.Println(r)
}
