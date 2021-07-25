// https://leetcode-cn.com/problems/two-sum-ii-input-array-is-sorted/
package main

import "fmt"

func impl1(numbers []int, target int) []int {
	var (
		hMap   map[int]int
		result []int
	)
	hMap = make(map[int]int)

	for i, ele := range numbers {
		if index, ok := hMap[ele]; ok {
			result = []int{index + 1, i + 1}
			break
		} else {
			hMap[target-ele] = i
		}
	}
	return result
}

func impl2(numbers []int, target int) []int {
	var (
		left   int
		right  int
		sum    int
		result []int
	)
	right = len(numbers) - 1
	for left < right {
		sum = numbers[left] + numbers[right]
		if sum == target {
			return []int{left + 1, right + 1}
		} else if sum > target {
			right--
		} else {
			left++
		}
	}
	return result
}

func twoSum(numbers []int, target int) []int {
	// return impl1(numbers, target)
	return impl2(numbers, target)
}

func main() {
	fmt.Println(twoSum([]int{2, 7, 11, 15}, 9))
	fmt.Println(twoSum([]int{2, 3, 4}, 6))
	fmt.Println(twoSum([]int{12, 13, 23, 28, 43, 44, 59, 60, 61, 68, 70, 86, 88, 92, 124, 125, 136, 168, 173, 173, 180, 199, 212, 221, 227, 230, 277, 282, 306, 314, 316, 321, 325, 328, 336, 337, 363, 365, 368, 370, 370, 371, 375, 384, 387, 394, 400, 404, 414, 422, 422, 427, 430, 435, 457, 493, 506, 527, 531, 538, 541, 546, 568, 583, 585, 587, 650, 652, 677, 691, 730, 737, 740, 751, 755, 764, 778, 783, 785, 789, 794, 803, 809, 815, 847, 858, 863, 863, 874, 887, 896, 916, 920, 926, 927, 930, 933, 957, 981, 997}, 542))
}
