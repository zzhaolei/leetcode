#![allow(unused)]
/// https://leetcode-cn.com/problems/zai-pai-xu-shu-zu-zhong-cha-zhao-shu-zi-lcof/

struct Solution;

impl Solution {
    fn impl_1(nums: Vec<i32>, target: i32) -> i32 {
        nums.iter().filter(|e| **e == target).count() as i32
    }
    fn impl_2(nums: Vec<i32>, target: i32) -> i32 {
        let mut r = 0;
        for v in nums.iter() {
            if *v == target {
                r += 1;
            }
        }
        r
    }
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        // Solution::impl_1(nums, target)
        Solution::impl_2(nums, target)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search() {
        assert_eq!(Solution::search(vec![5, 7, 7, 8, 8, 10], 8), 2);
        assert_eq!(Solution::search(vec![5, 7, 7, 8, 8, 10], 6), 0);
    }
}
fn main() {}
