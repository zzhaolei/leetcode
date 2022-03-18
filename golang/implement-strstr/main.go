package main

import (
	"fmt"
	"strings"
)

func strStr(haystack string, needle string) int {
	return strings.Index(haystack, needle)
}

func main() {
	fmt.Println(strStr("mississipipi", "issip"))
	fmt.Println(strStr("mississippi", "issip"))
	fmt.Println(strStr("hellllo", "ll"))
	fmt.Println(strStr("helllo", "ll"))
	fmt.Println(strStr("helllo", ""))
	fmt.Println(strStr("", ""))
	fmt.Println(strStr("", "1"))
}
