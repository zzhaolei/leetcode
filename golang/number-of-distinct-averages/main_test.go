package main

import "testing"

func TestDistinctAverages(t *testing.T) {
	if ans := distinctAverages([]int{4, 1, 4, 0, 3, 5}); ans != 2 {
		t.Errorf("非预期的值 2，%d", ans)
	}

	if ans := distinctAverages([]int{9, 5, 7, 8, 7, 9, 8, 2, 0, 7}); ans != 5 {
		t.Errorf("非预期的值 5，%d", ans)
	}

	if ans := distinctAverages([]int{1, 100}); ans != 1 {
		t.Errorf("非预期的值 1，%d", ans)
	}
}
