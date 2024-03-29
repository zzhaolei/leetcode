#![allow(unused)]

struct Solution;

impl Solution {
    fn impl_1(nums: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        let _ = nums
            .into_iter()
            .filter(|x| {
                if let Some(r) = map.get_mut(x) {
                    *r = 2;
                } else {
                    map.insert(*x, 1);
                }
                false
            })
            .collect::<Vec<i32>>();
        return map
            .into_iter()
            .filter(|(_, v)| *v != 2)
            .collect::<HashMap<i32, i32>>()
            .keys()
            .copied()
            .collect::<Vec<i32>>();
    }

    fn impl_2(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = 0;
        for &i in nums.iter() {
            ans ^= i;
        }

        let bit = ans & -ans;
        let mut num1 = 0;
        let mut num2 = 0;
        for x in nums.into_iter() {
            if x & bit == 0 {
                num1 ^= x;
            } else {
                num2 ^= x;
            }
        }
        vec![num1, num2]
    }

    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        Solution::impl_1(nums)
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn impl_1() {
        let r = Solution::impl_1(vec![1, 2, 3, 4, 2, 1]);
        assert_eq!(r.len(), 2);
        assert!(r.contains(&3));
        assert!(r.contains(&4));
    }

    #[test]
    fn impl_2() {
        let r = Solution::impl_2(vec![1, 2, 1, 3, 2, 5]);
        println!("{:?}", r);
        assert_eq!(r.len(), 2);
        assert!(r.contains(&3));
        assert!(r.contains(&5));
    }
}
fn main() {}
