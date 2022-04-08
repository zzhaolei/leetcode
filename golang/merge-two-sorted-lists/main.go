package main

import "fmt"

/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */

type ListNode struct {
	Val  int
	Next *ListNode
}

func mergeTwoLists(list1 *ListNode, list2 *ListNode) *ListNode {
	if list1 == nil {
		return list2
	}
	if list2 == nil {
		return list1
	}
	var (
		node, result *ListNode
	)
	if list1.Val < list2.Val {
		node = list1
		result = mergeTwoLists(list1.Next, list2)
	} else {
		node = list2
		result = mergeTwoLists(list2.Next, list1)
	}
	node.Next = result

	return node
}

func main() {
	list1 := &ListNode{
		Val: 1,
		Next: &ListNode{
			Val: 2,
			Next: &ListNode{
				Val:  4,
				Next: nil,
			},
		},
	}

	list2 := &ListNode{
		Val: 1,
		Next: &ListNode{
			Val: 3,
			Next: &ListNode{
				Val:  4,
				Next: nil,
			},
		},
	}
	v := mergeTwoLists(list1, list2)
	for {
		if v == nil {
			break
		}
		fmt.Println(v.Val)
		v = v.Next
	}
}
