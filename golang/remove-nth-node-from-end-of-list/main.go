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
	var list []*ListNode
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

func impl2(head *ListNode, n int) *ListNode {
	if head == nil {
		return head
	}
	if n == 1 && head.Next == nil {
		return nil
	}
	slowNode := head
	nodeNext := head
	s, f := 0, 0
	for nodeNext != nil {
		f++
		// 当遍历完成时，slowNode的next节点正好就是要删除的节点
		if f >= n+1 {
			if s != 0 {
				slowNode = slowNode.Next
			}
			s++
		}
		nodeNext = nodeNext.Next
	}
	if f == n {
		return head.Next
	}
	// 删除节点
	slowNode.Next = slowNode.Next.Next
	return head
}

func removeNthFromEnd(head *ListNode, n int) *ListNode {
	//return impl1(head, n)
	return impl2(head, n)
}

func main() {}
