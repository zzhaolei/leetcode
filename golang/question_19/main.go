// https://leetcode-cn.com/problems/remove-nth-node-from-end-of-list/
package main

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

func impl1(head *ListNode, n int) *ListNode {
	if head == nil {
		return nil
	}
	list := []*ListNode{}
	next := head
	for next != nil {
		list = append(list, next)
		next = next.Next
	}
	index := len(list) - 1 - n
	if index < 0 {
		if index == -1 && len(list) >= 2 {
			return list[1]
		}
		return nil
	}
	list[index].Next = list[index].Next.Next
	return head
}

func removeNthFromEnd(head *ListNode, n int) *ListNode {
	return impl1(head, n)
}

func main() {}
