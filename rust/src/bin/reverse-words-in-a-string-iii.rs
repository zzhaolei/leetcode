struct Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut ans = String::new();
        for i in s.split_whitespace() {
            let i = i.chars().rev().collect::<String>();
            ans.push_str(&i);
            ans.push(' ');
        }
        ans.trim().to_string()
    }
}

fn main() {
    println!(
        "{}",
        Solution::reverse_words("Let's take LeetCode contest".to_string())
    );
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::reverse_words("Let's take LeetCode contest".to_string()),
            "s'teL ekat edoCteeL tsetnoc"
        )
    }
}
