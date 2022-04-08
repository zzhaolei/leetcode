#![allow(unused)]
use std::usize;

struct Solution;

/// https://leetcode-cn.com/problems/remove-duplicates-from-sorted-array/
impl Solution {
    pub fn impl_1(nums: Vec<i32>, target: i32) -> i32 {
        for (i, v) in nums.iter().enumerate() {
            if target <= *v {
                return i as i32;
            }
        }
        nums.len() as i32
    }
    pub fn impl_2(nums: Vec<i32>, target: i32) -> i32 {
        nums.binary_search(&target).unwrap_or_else(|x| x) as i32
    }
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        // Solution::impl_1(nums, target)
        Solution::impl_2(nums, target)
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn impl_1() {
        assert_eq!(Solution::impl_1(vec![1, 3, 5, 6], 5), 2);
        assert_eq!(Solution::impl_1(vec![1, 3, 5, 6], 2), 1);
        assert_eq!(Solution::impl_1(vec![1, 3, 5, 6], 7), 4);
        assert_eq!(Solution::impl_1(vec![1, 3, 5, 6], 0), 0);
    }

    #[test]
    fn impl_2() {
        assert_eq!(Solution::impl_2(vec![1, 3, 5, 6], 5), 2);
        assert_eq!(Solution::impl_2(vec![1, 3, 5, 6], 2), 1);
        assert_eq!(Solution::impl_2(vec![1, 3, 5, 6], 7), 4);
        assert_eq!(Solution::impl_2(vec![1, 3, 5, 6], 0), 0);
    }
}
