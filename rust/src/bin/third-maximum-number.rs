#![allow(unused)]

/// https://leetcode-cn.com/problems/third-maximum-number/
struct Solution;

use std::collections::HashSet;

impl Solution {
    fn impl_1(nums: Vec<i32>) -> i32 {
        let mut nums = nums
            .into_iter()
            .collect::<HashSet<i32>>()
            .into_iter()
            .collect::<Vec<i32>>();
        nums.sort_by(|x, y| y.partial_cmp(x).unwrap());

        if nums.len() >= 3 {
            unsafe { return *nums.get_unchecked(2) }
        } else {
            unsafe { return *nums.get_unchecked(0) }
        }
        0
    }

    fn impl_2(nums: Vec<i32>) -> i32 {
        let mut set = HashSet::new();
        let num = i32::MIN; // 最小值
        let (mut max, mut mid, mut min) = (num, num, num);
        for i in nums.iter() {
            let i = *i;
            set.insert(i);
            if i > max {
                min = mid;
                mid = max;
                max = i;
            } else if i < max && i > mid {
                min = mid;
                mid = i;
            } else if i < mid && i > min {
                min = i;
            }
            println!("{}, {}, {}, {}", i, max, mid, min);
        }
        if set.len() >= 3 {
            min
        } else {
            max
        }
    }

    pub fn third_max(nums: Vec<i32>) -> i32 {
        Solution::impl_1(nums)
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn impl_1() {
        assert_eq!(Solution::impl_1(vec![3, 2, 1]), 1);
        assert_eq!(Solution::impl_1(vec![1, 2]), 2);
        assert_eq!(Solution::impl_1(vec![2, 2, 3, 1]), 1);
        assert_eq!(Solution::impl_1(vec![1, 2, -2147483648]), -2147483648);
    }

    #[test]
    fn impl_2() {
        println!("{}", Solution::impl_2(vec![3, 2, 1]));
        assert_eq!(Solution::impl_2(vec![3, 2, 1]), 1);
        assert_eq!(Solution::impl_2(vec![1, 2]), 2);
        assert_eq!(Solution::impl_2(vec![2, 2, 3, 1]), 1);
        assert_eq!(Solution::impl_2(vec![1, 2, -2147483648]), -2147483648);
    }
}
fn main() {}
