// https://leetcode-cn.com/problems/valid-anagram/
package main

import "fmt"

func isAnagram(s string, t string) bool {
	if len(s) != len(t) {
		return false
	}
	m := make(map[rune]int, len(s))
	for _, v := range s {
		if vv, ok := m[v]; ok {
			m[v] = vv + 1
		} else {
			m[v] = 1
		}
	}
	for _, v := range t {
		if vv, ok := m[v]; ok {
			if vv-1 < 0 {
				return false
			}
			m[v] = vv - 1
		} else {
			return false
		}
	}
	return true
}

func main() {
	fmt.Println(isAnagram("anagram", "nagaram"), true)
	fmt.Println(isAnagram("rat", "car"), false)
	fmt.Println(isAnagram("aacc", "ccac"), false)
}
