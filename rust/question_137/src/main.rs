use std::collections::HashMap;

/// https://leetcode-cn.com/problems/single-number-ii/

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

fn main() {
    assert_eq!(Solution::single_number(vec![2, 2, 3, 2]), 3);
    assert_eq!(Solution::single_number(vec![0, 1, 0, 1, 0, 1, 99]), 99);
}
