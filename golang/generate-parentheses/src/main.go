// https://leetcode-cn.com/problems/lian-biao-zhong-dao-shu-di-kge-jie-dian-lcof/
/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */

package main

import "fmt"

type ListNode struct {
	Val  int
	Next *ListNode
}

func impl1(head *ListNode, k int) *ListNode {
	s := make([]*ListNode, 0, k)
	next := head
	for next != nil {
		s = append(s, next)
		next = next.Next
	}
	if len(s) == 0 {
		return nil
	}
	return s[len(s)-k]
}

func impl2(head *ListNode, k int) *ListNode {
	// 两个都指向头部
	fast, slow := head, head

	// 快指针先走，让其领先 slow 指针 k 个节点
	for fast != nil && k > 0 {
		fast = fast.Next
		k--
	}
	// 两个指针同时遍历，当fast走到结尾时，slow即为需要查找的节点
	for fast != nil {
		fast = fast.Next
		slow = slow.Next
	}
	return slow
}

func impl3(head *ListNode, k int) *ListNode {
	var (
		s      int
		cursor *ListNode
	)
	cursor = head
	for cursor != nil {
		cursor = cursor.Next
		s++
	}
	cursor = head
	for i := 0; i < s-k; i++ {
		cursor = cursor.Next
	}
	return cursor
}

func getKthFromEnd(head *ListNode, k int) *ListNode {
	return impl1(head, k)
}

func main() {
	fmt.Println(getKthFromEnd(nil, 0))
}
