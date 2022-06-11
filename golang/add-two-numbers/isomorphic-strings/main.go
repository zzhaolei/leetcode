package main

import "fmt"

func isIsomorphic(s string, t string) bool {
	s_m := make(map[rune]rune)
	t_m := make(map[rune]rune)

	new_t := []rune(t)
	for i, c := range s {
		t_c := new_t[i]
		if v, ok := s_m[c]; ok && v != t_c {
			return false
		} else if v, ok := t_m[t_c]; ok && v != c {
			return false
		} else {
			s_m[c] = t_c
			t_m[t_c] = c
		}
	}
	return true
}

func main() {
	fmt.Println(isIsomorphic("a", "a"))
}
