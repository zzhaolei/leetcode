package main

import "fmt"

func main() {
	r := findAnagrams("cbaebabacd", "abc")
	fmt.Println(r)

	r = findAnagrams("abab", "ab")
	fmt.Println(r)

	r = findAnagrams("aaaaaaaa", "ab")
	fmt.Println(r)

	r = findAnagrams("aaaaaaaa", "aa")
	fmt.Println(r)
}

func findAnagrams(s, p string) (ans []int) {
	sLen, pLen := len(s), len(p)
	if sLen < pLen {
		return
	}

	var sCount, pCount [26]int
	for i, ch := range p {
		sCount[s[i]-'a']++
		pCount[ch-'a']++
	}
	if sCount == pCount {
		ans = append(ans, 0)
	}

	// 主要目的是判断 s[pLen:] 后面的数据是否满足要求
	// 1.从 sCount 中从头开始删除，然后将 i+pLen 加入到 sCount 中
	for i, ch := range s[:sLen-pLen] {
		sCount[ch-'a']--
		sCount[s[i+pLen]-'a']++
		if sCount == pCount {
			ans = append(ans, i+1)
		}
	}
	return
}
