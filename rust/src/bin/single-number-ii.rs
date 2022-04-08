#![allow(unused)]

/// https://leetcode-cn.com/problems/single-number-ii/
use std::collections::HashMap;

struct Solution;

impl Solution {
    fn impl_1(nums: Vec<i32>) -> i32 {
        let mut t: HashMap<i32, i32> = HashMap::new();
        for i in nums.iter() {
            *t.entry(*i).or_insert(0) += 1;
        }
        for (k, v) in t.into_iter() {
            if v == 1 {
                return k as i32;
            }
        }
        0
    }
    pub fn single_number(nums: Vec<i32>) -> i32 {
        Solution::impl_1(nums)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn impl_1() {
        assert_eq!(Solution::impl_1(vec![2, 2, 3, 2]), 3);
        assert_eq!(Solution::impl_1(vec![0, 1, 0, 1, 0, 1, 99]), 99);
    }
}
fn main() {}
