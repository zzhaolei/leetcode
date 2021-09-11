#![allow(unused)]
/// https://leetcode-cn.com/problems/single-number/submissions/
struct Solution;

impl Solution {
    fn impl1(nums: Vec<i32>) -> i32 {
        let mut r = 0;
        for v in nums.iter() {
            r ^= v;
        }
        r
    }

    fn impl2(mut nums: Vec<i32>) -> i32 {
        for i in 1..nums.len() {
            nums[0] ^= nums[i];
        }
        return nums[0];
    }

    fn impl3(nums: Vec<i32>) -> i32 {
        nums.iter().fold(0, |r, x| r ^ x)
    }

    pub fn single_number(nums: Vec<i32>) -> i32 {
        // Solution::impl1(nums)
        // Solution::impl2(nums)
        Solution::impl3(nums)
    }
}

fn main() {
    println!("{}, {}", Solution::single_number(vec![2, 2, 1]), 1);
}
