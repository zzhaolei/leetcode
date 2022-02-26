struct Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split_whitespace().rev().collect::<Vec<&str>>().join(" ")
    }
}

fn main() {
    println!("{}", Solution::reverse_words("the sky is blue".to_string()));
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::reverse_words("the sky is blue".to_string()),
            "blue is sky the"
        );
        assert_eq!(
            Solution::reverse_words("  hello world!  ".to_string()),
            "world! hello"
        );
        assert_eq!(
            Solution::reverse_words("a good   example".to_string()),
            "example good a"
        );
    }
}
