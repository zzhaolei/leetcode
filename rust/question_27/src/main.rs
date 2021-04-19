#![allow(unused)]
/// https://leetcode-cn.com/problems/remove-element/

struct Solution;

impl Solution {
    fn impl_1(nums: &mut Vec<i32>, val: i32) -> i32 {
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

    fn impl_2(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut idx = 0;
        let len = nums.len();
        for i in 0..len {
            if nums[i] != val {
                nums[idx] = nums[i];
                idx += 1;
            }
        }
        idx as i32
    }

    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        // Solution::impl_1(nums, val)
        Solution::impl_2(nums, val)
    }
}

fn main() {
    // let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
    let s = Solution::remove_element(&mut vec![0, 1, 2, 2, 3, 0, 4, 2], 2);
    assert_eq!(s, 5);
    let s = Solution::remove_element(&mut vec![0, 1, 2, 2, 2, 3, 0, 4, 2], 2);
    assert_eq!(s, 5);
}
