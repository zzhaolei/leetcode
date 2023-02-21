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

struct Solution;

impl Solution {
    fn recursion(head: &mut Option<Box<ListNode>>, vec: &mut Vec<i32>) {
        match head {
            Some(ref mut node) => {
                if let Some(_) = node.next {
                    Solution::recursion(&mut node.next.take(), vec);
                }
                vec.push(node.val);
            }
            None => return,
        }
    }
    pub fn reverse_print(mut head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut vec = Vec::with_capacity(10000);
        Solution::recursion(&mut head, &mut vec);
        vec
    }
}

fn main() {
    let head = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 3,
            next: Some(Box::new(ListNode::new(2))),
        })),
    }));
    let r = Solution::reverse_print(head);
    println!("{r:?}");
}
