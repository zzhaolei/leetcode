package main

import (
	"fmt"
)

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

func (l *ListNode) String() string {
	return fmt.Sprintf("%d, %v", l.Val, l.Next)
}

func addTwoNumbers(l1 *ListNode, l2 *ListNode) *ListNode {
	var (
		ten  int
		num  int
		head *ListNode
		list *ListNode
		next *ListNode
	)
	for l1 != nil || l2 != nil {
		num = 0
		if l1 != nil {
			num += l1.Val
			l1 = l1.Next
		}
		if l2 != nil {
			num += l2.Val
			l2 = l2.Next
		}
		num += ten
		next = &ListNode{
			Val:  num % 10,
			Next: nil,
		}
		if head == nil {
			head = next
			list = head
		} else {
			list.Next = next
			list = list.Next
		}
		ten = num / 10
	}
	if ten != 0 {
		list.Next = &ListNode{
			Val:  ten,
			Next: nil,
		}
	}
	return head
}

func main() {
	list1 := &ListNode{
		Val: 2,
		Next: &ListNode{
			Val: 4,
			Next: &ListNode{
				Val:  9,
				Next: nil,
			},
		},
	}

	list2 := &ListNode{
		Val: 5,
		Next: &ListNode{
			Val: 6,
			Next: &ListNode{
				Val: 4,
				Next: &ListNode{
					Val:  9,
					Next: nil,
				},
			},
		},
	}
	v := addTwoNumbers(list1, list2)
	fmt.Println(v)
}
