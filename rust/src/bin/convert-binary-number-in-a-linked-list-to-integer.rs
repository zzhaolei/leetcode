struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[allow(unused)]
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn impl1(head: Option<Box<ListNode>>) -> i32 {
        let mut s = String::new();
        let mut head = head;
        while let Some(node) = head {
            s.push_str(&node.val.to_string());
            head = node.next;
        }
        i32::from_str_radix(&s, 2).unwrap_or(0)
    }

    #[allow(unused)]
    pub fn impl2(head: Option<Box<ListNode>>) -> i32 {
        let mut head = head;
        let mut ans = 0;
        while let Some(node) = head {
            head = node.next;
            ans = node.val + ans * 2;
        }
        ans
    }

    pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
        Solution::impl1(head)
    }
}

fn main() {
    println!("{}", Solution::get_decimal_value(None));
}

#[cfg(test)]
mod tests {
    use crate::{ListNode, Solution};

    #[test]
    fn test_impl1() {
        assert_eq!(
            Solution::impl1(Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 0,
                    next: Some(Box::new(ListNode { val: 1, next: None })),
                })),
            }))),
            5
        );
        assert_eq!(
            Solution::impl1(Some(Box::new(ListNode { val: 0, next: None }))),
            0
        );
        assert_eq!(
            Solution::impl1(Some(Box::new(ListNode { val: 1, next: None }))),
            1
        );
        assert_eq!(
            Solution::impl1(Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode { val: 0, next: None }))
            }))),
            0
        );
    }

    #[test]
    fn test_impl2() {
        assert_eq!(
            Solution::impl2(Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 0,
                    next: Some(Box::new(ListNode { val: 1, next: None })),
                })),
            }))),
            5
        );
        assert_eq!(
            Solution::impl2(Some(Box::new(ListNode { val: 0, next: None }))),
            0
        );
        assert_eq!(
            Solution::impl2(Some(Box::new(ListNode { val: 1, next: None }))),
            1
        );
        assert_eq!(
            Solution::impl2(Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode { val: 0, next: None }))
            }))),
            0
        );
    }
}
