#![allow(unused)]

/// https://leetcode-cn.com/problems/third-maximum-number/
struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        let mut nums = nums
            .into_iter()
            .collect::<HashSet<i32>>()
            .into_iter()
            .collect::<Vec<i32>>();
        nums.sort_by(|x, y| y.partial_cmp(x).unwrap());

        if nums.len() >= 3 {
            unsafe { return *nums.get_unchecked(2) }
        } else {
            unsafe { return *nums.get_unchecked(0) }
        }
        return 0;
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn third_max() {
        // assert_eq!(Solution::third_max(vec![3, 2, 1]), 1);
        // assert_eq!(Solution::third_max(vec![1, 2]), 2);
        assert_eq!(Solution::third_max(vec![2, 2, 3, 1]), 1);
    }
}
