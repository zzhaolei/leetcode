package main

import "fmt"

func main() {
	r := maxArea([]int{1, 8, 6, 2, 5, 4, 8, 3, 7})
	fmt.Println(r)

	r = maxArea([]int{1, 1})
	fmt.Println(r)
}

func maxArea(height []int) int {
	var ans int
	for f, s := 0, len(height)-1; f < s; {
		if height[f] < height[s] {
			ans = max(ans, height[f]*(s-f))
			f += 1
		} else {
			ans = max(ans, height[s]*(s-f))
			s -= 1
		}
	}
	return ans
}
