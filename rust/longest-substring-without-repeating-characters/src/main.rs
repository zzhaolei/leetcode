struct Solution;

impl Solution {
    pub fn impl1(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }
        use std::collections::HashSet;
        let mut max = 0;
        let mut ans = 0;
        let mut set: HashSet<char> = HashSet::new();
        let chars = s.chars().collect::<Vec<char>>();

        for i in 0..s.len() {
            for &i in chars.iter().skip(i) {
                if !set.insert(i) {
                    break;
                }
                max += 1;
            }
            if max > ans {
                ans = max;
            }
            max = 0;
            set.clear();
        }
        ans
    }

    pub fn length_of_longest_substring(s: String) -> i32 {
        Solution::impl1(s)
    }
}

fn main() {
    println!(
        "{}",
        Solution::length_of_longest_substring("abcabcbb".to_string())
    );
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_impl1() {
        assert_eq!(Solution::test_impl1("abcabcbb".to_string()), 3);
        assert_eq!(Solution::test_impl1("bbbbb".to_string()), 1);
        assert_eq!(Solution::test_impl1("pwwkew".to_string()), 3);
    }
}
