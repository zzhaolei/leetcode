#![allow(unused)]
/// https://leetcode-cn.com/problems/remove-duplicates-from-sorted-list/

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
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match head {
            Some(mut head) => {
                if let Some(next) = head.next {
                    let next_n;
                    if head.val == next.val {
                        next_n = next.next;
                    } else {
                        next_n = Some(next);
                    }

                    let next = Solution::delete_duplicates(next_n);
                    if let Some(next) = next {
                        if head.val == next.val {
                            head.next = next.next;
                        } else {
                            head.next = Some(next);
                        }
                    } else {
                        head.next = None;
                    }
                }
                Some(head)
            }
            None => head,
        }
    }
}

fn main() {
    let l1 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode { val: 2, next: None })),
        })),
    }));
    let l2 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode { val: 1, next: None })),
        })),
    }));
    let l3 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode { val: 2, next: None })),
            })),
        })),
    }));

    println!("Result: {:?}", Solution::delete_duplicates(l1));
    println!("Result: {:?}", Solution::delete_duplicates(l2));
    println!("Result: {:?}", Solution::delete_duplicates(l3));
}
