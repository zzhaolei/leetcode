package main

import "fmt"

func isUnique(astr string) bool {
	for i := 0; i < len(astr)-1; i++ {
		v := astr[i]
		for j := i + 1; j < len(astr); j++ {
			if v == astr[j] {
				return false
			}
		}
	}
	return true
}

func main() {
	fmt.Println(isUnique("l1e1cod"))
	fmt.Println(isUnique("leEtcodE"))
	fmt.Println(isUnique("leEtcod"))
}
