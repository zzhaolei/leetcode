struct Solution;

impl Solution {
    pub fn impl1(s: String) -> i32 {
        s.trim_end_matches(' ')
            .split_whitespace()
            .rev()
            .next()
            .unwrap_or("")
            .len() as i32
    }

    #[allow(unused)]
    pub fn impl2(s: String) -> i32 {
        let mut ans = 0;
        for i in s.chars().rev() {
            if i == ' ' {
                if ans == 0 {
                    continue;
                }
                break;
            }
            ans += 1;
        }
        ans
    }

    pub fn length_of_last_word(s: String) -> i32 {
        Solution::impl1(s)
    }
}

fn main() {
    println!(
        "{}",
        Solution::length_of_last_word("Hello World".to_string())
    )
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_impl1() {
        assert_eq!(Solution::impl1("Hello World".to_string()), 5);
        assert_eq!(
            Solution::impl1("   fly me   to   the moon  ".to_string()),
            4
        );
        assert_eq!(Solution::impl1("luffy is still joyboy".to_string()), 6);
    }

    #[test]
    fn test_impl2() {
        assert_eq!(Solution::impl2("Hello World".to_string()), 5);
        assert_eq!(
            Solution::impl2("   fly me   to   the moon  ".to_string()),
            4
        );
        assert_eq!(Solution::impl2("luffy is still joyboy".to_string()), 6);
    }
}
