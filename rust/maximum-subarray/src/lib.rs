#![allow(unused)]

/// https://leetcode-cn.com/problems/maximum-subarray/

struct Solution;

impl Solution {
    fn impl1(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }

        let mut max = nums[0];
        let mut find;
        for i in 0..nums.len() {
            find = nums[i];
            for j in (i + 1)..nums.len() {
                if nums[j] > max {
                    max = nums[j];
                }
                find = find + nums[j];
                if find > max {
                    max = find;
                }
            }
        }
        return max;
    }

    fn impl2(nums: Vec<i32>) -> i32 {
        let mut ans = nums[0];
        let mut sum = 0;
        for i in nums.into_iter() {
            if sum > 0 {
                sum += i;
            } else {
                sum = i;
            }
            if sum > ans {
                ans = sum;
            }
        }
        ans
    }

    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        Solution::impl1(nums)
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn impl1() {
        assert_eq!(Solution::impl1(vec![-2, 1]), 1);
        assert_eq!(Solution::impl1(vec![-10000]), -10000);
        assert_eq!(Solution::impl1(vec![1]), 1);
        assert_eq!(Solution::impl1(vec![0]), 0);
        assert_eq!(Solution::impl1(vec![-1]), -1);
        assert_eq!(Solution::impl1(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
    }

    #[test]
    fn impl2() {
        assert_eq!(Solution::impl2(vec![-2, 1]), 1);
        assert_eq!(Solution::impl2(vec![-10000]), -10000);
        assert_eq!(Solution::impl2(vec![1]), 1);
        assert_eq!(Solution::impl2(vec![0]), 0);
        assert_eq!(Solution::impl2(vec![-1]), -1);
        assert_eq!(Solution::impl2(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
    }
}
