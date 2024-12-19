package main

import "fmt"

func main() {
	r := stableMountains([]int{1, 2, 3, 4, 5}, 2)
	fmt.Println(r)

	r = stableMountains([]int{10, 1, 10, 1, 10}, 3)
	fmt.Println(r)

	r = stableMountains([]int{10, 1, 10, 1, 10}, 10)
	fmt.Println(r)
}

func stableMountains(height []int, threshold int) (ans []int) {
	for i := 1; i < len(height); i++ {
		if height[i-1] > threshold {
			ans = append(ans, i)
		}
	}
	return
}