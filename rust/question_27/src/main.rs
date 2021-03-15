#![allow(unused)]
/// https://leetcode-cn.com/problems/remove-element/

struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut idx = 0;
        while idx < nums.len() {
            if nums[idx] == val {
                nums.remove(idx);
            } else {
                idx += 1;
            }
        }
        idx as i32
    }
}

fn main() {
    let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
    let s = Solution::remove_element(&mut nums, 2);
    println!("{}", s)
}
