#![allow(unused)]

struct Solution;

impl Solution {
    fn is_palindromic(s: &str) -> bool {
        let chars = s.as_bytes();
        let end = s.len();
        for i in 0..end / 2 {
            if chars[i] != chars[end - i - 1] {
                return false;
            }
        }
        true
    }
    pub fn impl1(s: String) -> String {
        if s.len() <= 1 || Solution::is_palindromic(&s) {
            return s;
        }
        let mut ans = "";
        for i in 0..s.len() - 1 {
            for j in i + 1..s.len() {
                let tmp = &s[i..=j];
                if Solution::is_palindromic(tmp) && ans.len() < tmp.len() {
                    ans = tmp;
                }
            }
        }
        if ans.is_empty() {
            s.chars().next().unwrap().to_string()
        } else {
            ans.to_string()
        }
    }

    pub fn impl2(s: String) -> String {
        if s.len() <= 1 {
            return s;
        }

        for i in (0..s.len() + 1).rev() {
            for j in (0..s.len() - i + 1).rev() {
                let tmp = &s[j..j + i];
                if Solution::is_palindromic(tmp) {
                    return tmp.to_string();
                }
            }
        }
        s.chars().next().unwrap().to_string()
    }

    pub fn longest_palindrome(s: String) -> String {
        Solution::impl1(s)
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_impl1() {
        assert_eq!(Solution::impl1("babad".to_string()), "bab");
        assert_eq!(Solution::impl1("cbbd".to_string()), "bb");
        assert_eq!(Solution::impl1("a".to_string()), "a");
        assert_eq!(Solution::impl1("aa".to_string()), "aa");
        assert_eq!(Solution::impl1("ac".to_string()), "a");
        assert_eq!(Solution::impl1("acb".to_string()), "a");
    }

    #[test]
    fn test_impl2() {
        assert_eq!(Solution::impl2("babad".to_string()), "aba");
        assert_eq!(Solution::impl2("cbbd".to_string()), "bb");
        assert_eq!(Solution::impl2("a".to_string()), "a");
        assert_eq!(Solution::impl2("aa".to_string()), "aa");
        assert_eq!(Solution::impl2("ac".to_string()), "c");
        assert_eq!(Solution::impl2("acb".to_string()), "b");
    }
}
