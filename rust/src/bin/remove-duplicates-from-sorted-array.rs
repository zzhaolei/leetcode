#![allow(unused)]
use std::usize;

struct Solution;

/// https://leetcode-cn.com/problems/remove-duplicates-from-sorted-array/
impl Solution {
    pub fn impl_1(nums: &mut Vec<i32>) -> i32 {
        nums.dedup();
        nums.len() as i32
    }
    pub fn impl_2(nums: &mut Vec<i32>) -> i32 {
        let mut dup: Option<i32> = None;
        nums.retain(|v| {
            if let Some(d) = dup {
                if d == *v {
                    return false;
                } else {
                    dup = Some(*v);
                }
            } else {
                dup = Some(*v);
            }
            true
        });
        nums.len() as i32
    }
    pub fn impl_3(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut dup = nums[0];
        let mut idx: usize = 1;
        while idx < nums.len() {
            let v = nums[idx];
            if v == dup {
                nums.remove(idx);
            } else {
                dup = v;
                idx += 1;
            }
        }

        idx as i32
    }
    pub fn impl_4(nums: &mut [i32]) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut i = 0;
        for j in 1..nums.len() {
            if nums[i] != nums[j] {
                i += 1;
                nums[i] = nums[j];
            }
        }
        i as i32 + 1
    }
    pub fn remove_duplicates(nums: &mut [i32]) -> i32 {
        // Self::impl_1(nums)
        // Self::impl_2(nums)
        // Self::impl_3(nums)
        Self::impl_4(nums)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn impl_1() {
        let mut n = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let r = Solution::impl_1(&mut n) as usize;
        assert_eq!(&n[..r], &[0, 1, 2, 3, 4]);
    }

    #[test]
    fn impl_2() {
        let mut n = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let r = Solution::impl_2(&mut n) as usize;
        assert_eq!(&n[..r], &[0, 1, 2, 3, 4]);
    }

    #[test]
    fn impl_3() {
        let mut n = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let r = Solution::impl_3(&mut n) as usize;
        assert_eq!(&n[..r], &[0, 1, 2, 3, 4]);
    }

    #[test]
    fn impl_4() {
        let mut n = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let r = Solution::impl_4(&mut n) as usize;
        assert_eq!(&n[..r], &[0, 1, 2, 3, 4]);
    }
}
fn main() {}
