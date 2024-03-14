package main

import (
	"fmt"
	"strings"
)

func maximumOddBinaryNumber(s string) string {
	one := strings.ReplaceAll(s, "0", "")
	zero := strings.ReplaceAll(s, "1", "")
	return strings.Replace(one, "1", "", 1) + zero + "1"
}

func main() {
	r := maximumOddBinaryNumber("001100")
	fmt.Println(r == "100001")
}
