#![allow(unused)]

/// https://leetcode-cn.com/problems/intersection-of-two-arrays/submissions/

struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let nums2 = nums2.iter().collect::<HashSet<_>>();
        nums1
            .into_iter()
            .filter(|x| nums2.contains(x))
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intersection() {
        assert_eq!(
            Solution::intersection(vec![1, 2, 2, 1], vec![2, 2]),
            vec![2]
        );
        let r = Solution::intersection(vec![4, 9, 5], vec![9, 4, 9, 8, 4]);
        assert!(r == vec![9, 4] || r == vec![4, 9]);
    }
}
fn main() {}
