#![allow(unused)]

struct Solution;

impl Solution {
    fn test(self) -> Self {
        println!("1");
        if (true) {
            return self;
        }

        self
    }
    pub fn impl1(nums: Vec<i32>, k: i32) -> bool {
        for i in 0..nums.len() - 1 {
            for j in i + 1..nums.len() {
                if nums[i] == nums[j] && (i as i32 - j as i32).abs() <= k {
                    return true;
                }
            }
        }
        false
    }

    pub fn impl2(nums: Vec<i32>, k: i32) -> bool {
        use std::collections::HashMap;
        let mut map: HashMap<i32, usize> = HashMap::new();

        for (i, v) in nums.iter().enumerate() {
            if let Some(x) = map.get_mut(v) {
                if (i as i32 - *x as i32).abs() <= k {
                    return true;
                }
                *x = i;
            } else {
                map.insert(*v, i);
            }
        }
        false
    }

    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        Solution::impl1(nums, k)
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_impl1() {
        let s = Solution;
        // Solution::test("");

        s.test();
        assert!(Solution::impl1(vec![1, 2, 3, 1], 3));
        assert!(Solution::impl1(vec![1, 0, 1, 1], 1));
        assert!(!Solution::impl1(vec![1, 2, 3, 1, 2, 3], 2));
    }

    #[test]
    fn test_impl2() {
        assert!(Solution::impl2(vec![1, 2, 3, 1], 3));
        assert!(Solution::impl2(vec![1, 0, 1, 1], 1));
        assert!(!Solution::impl2(vec![1, 2, 3, 1, 2, 3], 2));
    }
}
fn main() {}
