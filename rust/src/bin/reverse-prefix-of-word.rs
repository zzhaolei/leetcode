struct Solution;

impl Solution {
    pub fn impl1(word: String, ch: char) -> String {
        if let Some(i) = word.find(ch) {
            let mut s = String::new();
            s.push_str(&(&word[..=i]).chars().rev().collect::<String>());
            s.push_str(&word[i + 1..]);
            s
        } else {
            word
        }
    }

    #[allow(unused)]
    pub fn impl2(word: String, ch: char) -> String {
        if let Some(i) = word.find(ch) {
            (&word[..=i]).chars().rev().collect::<String>() + &word[i + 1..]
        } else {
            word
        }
    }

    pub fn reverse_prefix(word: String, ch: char) -> String {
        Solution::impl1(word, ch)
    }
}

fn main() {
    println!("{}", Solution::reverse_prefix("abcdefd".to_string(), 'd'));
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_impl1() {
        assert_eq!(
            Solution::reverse_prefix("abcdefd".to_string(), 'z'),
            "abcdefd"
        );
        assert_eq!(
            Solution::reverse_prefix("abcdefd".to_string(), 'd'),
            "dcbaefd"
        );
        assert_eq!(
            Solution::reverse_prefix("xyxzxe".to_string(), 'z'),
            "zxyxxe"
        );
        assert_eq!(Solution::reverse_prefix("abcd".to_string(), 'z'), "abcd");
    }

    #[test]
    fn test_impl2() {
        assert_eq!(Solution::impl2("abcdefd".to_string(), 'z'), "abcdefd");
        assert_eq!(Solution::impl2("abcdefd".to_string(), 'd'), "dcbaefd");
        assert_eq!(Solution::impl2("xyxzxe".to_string(), 'z'), "zxyxxe");
        assert_eq!(Solution::impl2("abcd".to_string(), 'z'), "abcd");
    }
}
