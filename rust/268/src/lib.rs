#![allow(unused)]

/// https://leetcode-cn.com/problems/missing-number/submissions/

struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut map: HashMap<&i32, Option<&i32>> = HashMap::new();
        for i in nums.iter() {
            map.insert(i, Some(i));
        }
        for i in 0..=nums.len() {
            if let None = map.get(&(i as i32)) {
                return i as i32;
            }
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_number() {
        assert_eq!(Solution::missing_number(vec![3, 0, 1]), 2);
        assert_eq!(Solution::missing_number(vec![0, 1]), 2);
        assert_eq!(Solution::missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]), 8);
        assert_eq!(Solution::missing_number(vec![0]), 1);
    }
}
