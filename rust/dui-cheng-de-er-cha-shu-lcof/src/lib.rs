// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution;

use std::cell::{Ref, RefCell};
use std::rc::Rc;

impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            return true;
        }
        // let root = root.unwrap().get_mut();
        let root = root.as_ref().unwrap().borrow_mut();
        Solution::check(root.left.clone(), root.right.clone())
    }

    pub fn check(
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if left.is_none() && right.is_none() {
            return true;
        }
        if (left.is_some() && right.is_none()) || (left.is_none() && right.is_some()) {
            return false;
        }
        let left = left.as_ref().unwrap().borrow_mut();
        let right = right.as_ref().unwrap().borrow_mut();
        // 比较两者的值
        // 左节点下的左节点 和 右节点下的右节点比较
        // 左节点下的右节点 和 右节点下的左节点比较
        left.val == right.val
            && Solution::check(left.left.clone(), right.right.clone())
            && Solution::check(left.right.clone(), right.left.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_symmetric() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
