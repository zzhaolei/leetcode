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
    pub fn remove(head: Option<Box<ListNode>>, n: i32, i: i32) -> (Option<Box<ListNode>>, i32) {
        if let Some(node) = head {
            let (next, count) = Solution::remove(node.next, n, i);
            if count + 1 == n {
                return (next, count + 1);
            }
            (
                Some(Box::new(ListNode {
                    val: node.val,
                    next,
                })),
                count + 1,
            )
        } else {
            (None, i)
        }
    }

    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        Solution::remove(head, n, 0).0
    }
}

fn main() {
    println!("{:?}", Solution::remove_nth_from_end(None, 0));
}
