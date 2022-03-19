package main

import (
	"bytes"
	"fmt"
)

func count(n int, i int, say string) string {
	if n == i {
		return say
	}

	var (
		buffer bytes.Buffer
		c      int32 = 49
		v            = int32(say[0])
	)
	for _, j := range say[1:] {
		if v == j {
			c++
		} else {
			buffer.WriteRune(c)
			buffer.WriteRune(v)
			c, v = 49, j
		}
	}
	buffer.WriteRune(c)
	buffer.WriteRune(v)
	return count(n, i+1, buffer.String())
}

func countAndSay(n int) string {
	return count(n, 1, "1")
}

func main() {
	fmt.Println(countAndSay(1) == "1")
	fmt.Println(countAndSay(2) == "11")
	fmt.Println(countAndSay(3) == "21")
	fmt.Println(countAndSay(4) == "1211")
	fmt.Println(countAndSay(5) == "111221")
}
