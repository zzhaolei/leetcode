#![allow(unused)]
//! 将两个升序链表合并为一个新的 升序 链表并返回。新链表是通过拼接给定的两个链表的所有节点组成的。
//!
//! # Examples
//! ```text
//! 输入：l1 = [1,2,4], l2 = [1,3,4]
//! 输出：[1,1,2,3,4,4]
//!
//! 输入：l1 = [], l2 = []
//! 输出：[]
//!
//! 输入：l1 = [], l2 = [0]
//! 输出：[0]
//! ```
//!

#[allow(unused)]
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;

impl Solution {
    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (Some(l1), Some(l2)) => {
                let mut nodes;
                if l1.val < l2.val {
                    nodes = l1.clone();
                    let r = Solution::merge_two_lists(l1.next, Some(l2));
                    nodes.next = r;
                } else {
                    nodes = l2.clone();
                    let r = Solution::merge_two_lists(l2.next, Some(l1));
                    nodes.next = r;
                }
                Some(nodes)
            }
            (None, Some(l2)) => {
                Some(l2)
            }
            (Some(l1), None) => {
                Some(l1)
            }
            (None, None) => {
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
