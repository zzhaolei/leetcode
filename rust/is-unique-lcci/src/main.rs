struct Solution;

impl Solution {
    pub fn is_unique(astr: String) -> bool {
        for (i, v) in astr.chars().enumerate() {
            for vv in astr.chars().skip(i + 1) {
                if v == vv {
                    return false;
                }
            }
        }
        return true;
    }
}

fn main() {
    println!("{}", Solution::is_unique("leetcode".to_string()));
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_impl1() {
        assert_eq!(Solution::is_unique("leetcode".to_string()), false);
        assert_eq!(Solution::is_unique("lei".to_string()), true);
        assert_eq!(Solution::is_unique("lei11".to_string()), false);
        assert_eq!(Solution::is_unique("Aa".to_string()), true);
        assert_eq!(Solution::is_unique("AA".to_string()), false);
    }
}
