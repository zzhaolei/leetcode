#![allow(unused)]
/// https://leetcode-cn.com/problems/number-of-1-bits/

struct Solution;

impl Solution {
    #[allow(non_snake_case)]
    pub fn hammingWeight(n: u32) -> i32 {
        let bin = format!("{:b}", n);
        let bin = bin
            .chars()
            .filter(|c| if *c == '1' { true } else { false })
            .collect::<Vec<char>>();
        bin.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hamming_weight() {
        assert_eq!(Solution::hammingWeight(11), 3);
    }
}
