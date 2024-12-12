package main

import (
	"fmt"
	"sort"
)

func main() {
	r := threeSum2([]int{-1, 0, 1, 2, -1, -4})
	fmt.Println(r)

	r = threeSum2([]int{0, 0, 0})
	fmt.Println(r)

	r = threeSum2([]int{-1, 0, 1, 2, -1, -4, -2, -3, 3, 0, 4})
	fmt.Println(r)
}

func threeSum(nums []int) [][]int {
	set := make(map[int]int)
	for i, v := range nums {
		set[v] = i
	}
	if len(set) == 1 {
		if v := nums[0]; v*3 == 0 {
			return [][]int{{v, v, v}}
		}
		return [][]int{}
	}

	m := make(map[[3]int]struct{})
	for i := 0; i < len(nums)-2; i++ {
		for j := i + 1; j < len(nums)-1; j++ {
			v := 0 - nums[i] - nums[j]
			if idx, ok := set[v]; ok && idx != i && idx != j {
				slice := []int{nums[i], nums[j], v}
				sort.Ints(slice)
				key := [3]int{slice[0], slice[1], slice[2]}
				m[key] = struct{}{}
			}
		}
	}
	var ans [][]int
	for k := range m {
		ans = append(ans, k[:])
	}
	return ans
}

func threeSum2(nums []int) [][]int {
	sort.Ints(nums) // 先排序

	var ans [][]int
	for i := 0; i < len(nums); i++ {
		if i > 0 && nums[i] == nums[i-1] { // 判断当前值和前一个值是否相等，相等则处理过，忽略
			continue
		}

		for l, r := i+1, len(nums)-1; l < r; { // 计算后半截数据
			sum := nums[l] + nums[r]
			if sum == -nums[i] { // 如果左右两个指针和当前 -i 相等，说明三者相加=0
				ans = append(ans, []int{nums[i], nums[l], nums[r]})
				l++ // 和下方的 for 配合，判断 l 值和 l+1 值是否是一个，如果是，则移动到 l+2 上去
				r-- // r 和 l 的逻辑类似
				for l < r && nums[l] == nums[l-1] {
					l++
				}
				for l < r && nums[r] == nums[r+1] {
					r--
				}
			} else if sum < -nums[i] { // 如果 l + r 的值小于 i，说明当前 l 的值太小了，应该移动 l+1
				l++
			} else { // 和移动 l 的逻辑类似
				r--
			}
		}
	}
	return ans
}
