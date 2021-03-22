#![allow(unused)]
/// https://leetcode-cn.com/problems/number-of-1-bits/

struct Solution;

impl Solution {
    pub fn hammingWeight(n: u32) -> i32 {
        let bin = format!("{:b}", n);
        let bin = bin
            .chars()
            .filter(|c| if *c == '1' { true } else { false })
            .collect::<Vec<char>>();
        bin.len() as i32
    }
}

fn main() {
    Solution::hammingWeight(11);
}
