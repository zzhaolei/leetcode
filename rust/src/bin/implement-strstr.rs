#![allow(unused)]
/// https://leetcode-cn.com/problems/implement-strstr/

struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.is_empty() {
            return 0;
        }
        match haystack.find(&needle) {
            Some(idx) => idx as i32,
            None => -1,
        }
    }
}

#[cfg(test)]
mod tests {}
fn main() {}
