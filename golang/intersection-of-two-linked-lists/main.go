package main

import (
	"fmt"
	"unsafe"
)

/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */

// ListNode 链表节点
type ListNode struct {
	Val  int
	Next *ListNode
}

func getIntersectionNode(headA, headB *ListNode) *ListNode {
	if headA == nil || headB == nil {
		return nil
	}
	set := make(map[uintptr]struct{})
	for headA != nil {
		set[uintptr(unsafe.Pointer(headA))] = struct{}{}
		headA = headA.Next
	}

	for headB != nil {
		if _, ok := set[uintptr(unsafe.Pointer(headB))]; ok {
			return headB
		}
		headB = headB.Next
	}
	return nil
}

func main() {
	hA := &ListNode{
		Val: 4,
		Next: &ListNode{
			Val: 1,
			Next: &ListNode{
				Val: 8,
				Next: &ListNode{
					Val: 4,
					Next: &ListNode{
						Val:  5,
						Next: nil,
					},
				},
			},
		},
	}
	hB := &ListNode{
		Val: 5,
		Next: &ListNode{
			Val: 6,
			Next: &ListNode{
				Val: 1,
				Next: &ListNode{
					Val: 8,
					Next: &ListNode{
						Val: 4,
						Next: &ListNode{
							Val:  5,
							Next: nil,
						},
					},
				},
			},
		},
	}

	r := getIntersectionNode(hA, hB)
	fmt.Printf("%#v\n", r)
}
