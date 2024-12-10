package main

import (
	"fmt"
	"sort"
	"strings"
)

func main() {
	r := groupAnagrams([]string{"eat", "tea", "tan", "ate", "nat", "bat"})
	fmt.Println(r)

	r = groupAnagrams2([]string{"cab", "tin", "pew", "duh", "may", "ill", "buy", "bar", "max", "doc"})
	fmt.Println(r)
}

func groupAnagrams(strs []string) [][]string {
	m := make(map[string][]string, len(strs))
	for _, v := range strs {
		newV := strings.Split(v, "")
		sort.Strings(newV)
		key := strings.Join(newV, "")
		m[key] = append(m[key], v)
	}

	var ans [][]string
	for _, v := range m {
		ans = append(ans, v)
	}
	return ans
}

func groupAnagrams2(strs []string) [][]string {
	m := make(map[[26]int][]string, len(strs))
	for _, v := range strs {
		var cnt [26]int
		for _, b := range v {
			cnt[b-'a']++
		}
		m[cnt] = append(m[cnt], v)
	}

	var ans [][]string
	for _, v := range m {
		ans = append(ans, v)
	}
	return ans
}
