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

func lengthOfLongestSubstring(s string) int {
	return impl1(s)
}

func main() {
	fmt.Println(lengthOfLongestSubstring("abcabcbb"), 3)
	fmt.Println(lengthOfLongestSubstring("bbbbb"), 1)
	fmt.Println(lengthOfLongestSubstring("pwwkew"), 3)
	fmt.Println(lengthOfLongestSubstring("aab"), 2)
	fmt.Println(lengthOfLongestSubstring("dvdf"), 3)
	fmt.Println(lengthOfLongestSubstring(""), 0)
}
