struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn _new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut head = head;
        let mut v: Vec<i32> = Vec::new();
        while let Some(node) = head {
            v.push(node.val);
            head = node.next;
        }
        for i in 0..v.len() / 2 {
            if v[i] != v[v.len() - i - 1] {
                return false;
            }
        }
        true
    }
}

fn main() {
    println!("{}", Solution::is_palindrome(None));
}

#[cfg(test)]
mod tests {
    use crate::{ListNode, Solution};

    #[test]
    fn test_impl1() {
        let node = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode { val: 1, next: None })),
                })),
            })),
        }));
        assert!(Solution::is_palindrome(node));
        let node = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode { val: 2, next: None })),
        }));
        assert!(!Solution::is_palindrome(node));
    }
}
