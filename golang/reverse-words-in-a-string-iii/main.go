// https://leetcode-cn.com/problems/reverse-words-in-a-string-iii/
package main

import (
	"fmt"
	"strings"
)

func reverseWords(s string) string {
	var (
		r []string
		t []byte
	)
	r = strings.Split(s, " ")
	for i, v := range r {
		t = []byte(v)
		for left, right := 0, len(t)-1; left < len(t)/2; left, right = left+1, right-1 {
			t[left], t[right] = t[right], t[left]
		}
		r[i] = string(t)
	}
	return strings.Join(r, " ")
}

func main() {
	fmt.Println(reverseWords("Let's take LeetCode contest"))
}
