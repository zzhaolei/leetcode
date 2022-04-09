#![allow(unused)]
/// https://leetcode-cn.com/problems/number-of-1-bits/

struct Solution;

impl Solution {
    fn impl1(n: u32) -> i32 {
        let bin = format!("{:b}", n);
        let bin = bin.chars().filter(|c| *c == '1').count();
        bin as i32
    }

    fn impl2(mut n: u32) -> i32 {
        let mut ans = 0;
        while n != 0 {
            if n & 1 != 0 {
                ans += 1;
            }
            n >>= 1;
        }
        ans
    }

    #[allow(non_snake_case)]
    pub fn hammingWeight(n: u32) -> i32 {
        Solution::impl1(n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hamming_weight() {
        assert_eq!(Solution::hammingWeight(11), 3);
        assert_eq!(Solution::impl1(11), 3);
        assert_eq!(Solution::impl2(11), 3);
    }
}

fn main() {}
