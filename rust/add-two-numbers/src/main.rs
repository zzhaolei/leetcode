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
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l1 = l1;
        let mut l2 = l2;
        let mut ans: Option<Box<ListNode>> = None;
        let mut p = &mut ans;
        let mut ten = 0;
        while l1.is_some() || l2.is_some() {
            let mut num = 0;
            if let Some(node) = l1 {
                num += node.val;
                l1 = node.next;
            }
            if let Some(node) = l2 {
                num += node.val;
                l2 = node.next;
            }
            num += ten;
            *p = Some(Box::new(ListNode::new(num % 10)));
            if let Some(node) = p {
                p = &mut node.next;
            }
            ten = num / 10;
        }
        if ten != 0 {
            *p = Some(Box::new(ListNode::new(ten)));
        }
        ans
    }
}

fn main() {
    let l1 = Some(Box::new(ListNode {
        val: 2,
        next: Some(Box::new(ListNode {
            val: 4,
            next: Some(Box::new(ListNode { val: 3, next: None })),
        })),
    }));
    let l2 = Some(Box::new(ListNode {
        val: 5,
        next: Some(Box::new(ListNode {
            val: 6,
            next: Some(Box::new(ListNode { val: 4, next: None })),
        })),
    }));
    println!("{:?}", Solution::add_two_numbers(l1, l2));
}

#[cfg(test)]
mod tests {
    use crate::{ListNode, Solution};

    #[test]
    fn test() {
        let l1 = Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode { val: 3, next: None })),
            })),
        }));
        let l2 = Some(Box::new(ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 6,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        }));
        assert_eq!(
            Solution::add_two_numbers(l1, l2),
            Some(Box::new(ListNode {
                val: 7,
                next: Some(Box::new(ListNode {
                    val: 0,
                    next: Some(Box::new(ListNode { val: 8, next: None })),
                })),
            }))
        )
    }
}
