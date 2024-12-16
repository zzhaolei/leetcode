// https://leetcode-cn.com/problems/longest-substring-without-repeating-characters/
package main

import "fmt"

func impl1(s string) int {
	var (
		m       = make(map[rune]struct{})
		maxLen  = 0
		nextLen = 0
	)
	for i := range s {
		for _, v := range s[i:] {
			if _, ok := m[v]; ok {
				m = make(map[rune]struct{}, maxLen)
				break
			}
			nextLen += 1
			m[v] = struct{}{}
		}
		if nextLen > maxLen {
			maxLen = nextLen
		}
		nextLen = 0
	}
	return maxLen
}

func impl2(s string) int {
	m := make(map[byte]int)
	r, ans := -1, 0
	for i := 0; i < len(s); i++ {
		if i != 0 {
			delete(m, s[i-1])
		}
		for r+1 < len(s) && m[s[r+1]] == 0 {
			m[s[r+1]] += 1
			r++
		}
		ans = max(ans, r+1-i)
	}

	return ans
}

func lengthOfLongestSubstring(s string) int {
	return impl2(s)
}

func main() {
	fmt.Println(lengthOfLongestSubstring("abcabcbb"), 3)
	fmt.Println(lengthOfLongestSubstring("bbbbb"), 1)
	fmt.Println(lengthOfLongestSubstring("pwwkew"), 3)
	fmt.Println(lengthOfLongestSubstring("aab"), 2)
	fmt.Println(lengthOfLongestSubstring("dvdf"), 3)
	fmt.Println(lengthOfLongestSubstring(""), 0)
}
