struct Solution;

// Definition for singly-linked list.
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

impl Solution {
    pub fn get_kth_from_end(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let (mut fast, mut slow) = (head.clone(), head);

        // 先让快指针领先k个元素
        let mut k = k;
        while fast.is_some() && k > 0 {
            fast = fast.unwrap().next;
            k -= 1;
        }

        while fast.is_some() {
            slow = slow.unwrap().next;
            fast = fast.unwrap().next;
        }
        slow
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::get_kth_from_end(Some(Box::new(ListNode::new(1))), 2)
    );
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_impl1() {
        let head = Some(Box::new(ListNode::new(1)));
        assert_eq!(
            Solution::get_kth_from_end(head, 2),
            Some(Box::new(ListNode::new(1)))
        );
    }
}
