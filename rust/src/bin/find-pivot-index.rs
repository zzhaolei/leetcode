#![allow(unused)]

/// https://leetcode-cn.com/submissions/detail/141823223/
struct Solution;

impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = nums.iter().sum::<i32>();
        for (i, v) in nums.iter().enumerate() {
            right -= v;
            if left == right {
                return i as i32;
            }
            left += v;
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn pivot_index() {
        assert_eq!(Solution::pivot_index(vec![1, 7, 3, 6, 5, 6]), 3);
        assert_eq!(Solution::pivot_index(vec![1, 2, 3]), -1);
        assert_eq!(Solution::pivot_index(vec![2, 1, -1]), 0);
    }
}
fn main() {}
