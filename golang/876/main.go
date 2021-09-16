// https://leetcode-cn.com/problems/middle-of-the-linked-list/
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

func impl1(head *ListNode) *ListNode {
	var (
		index int
		nodes []*ListNode
	)

	next := head
	for next != nil {
		nodes = append(nodes, next)
		index++
		next = next.Next
	}
	return nodes[index/2]
}

func impl2(head *ListNode) *ListNode {
	var (
		last *ListNode = head
		fast *ListNode = head
	)
	for fast != nil && fast.Next != nil {
		fast = fast.Next.Next
		last = last.Next
	}
	return last
}

func middleNode(head *ListNode) *ListNode {
	// return impl1(head)
	return impl2(head)
}

func main() {
	middleNode(&ListNode{
		Val: 1,
		Next: &ListNode{
			Val:  2,
			Next: nil,
		},
	})
}
