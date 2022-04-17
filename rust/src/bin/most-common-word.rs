struct Solution;

impl Solution {
    pub fn most_common_word(paragraph: String, banned: Vec<String>) -> String {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        let mut paragraph = paragraph.to_lowercase();
        for i in ["!", "?", "\'", ";", ",", "."] {
            paragraph = paragraph.replace(i, " ");
        }
        for i in paragraph.split_whitespace() {
            let i = i.to_string();
            if banned.contains(&i) {
                continue;
            }
            if let Some(v) = map.get_mut(&i) {
                *v += 1;
            } else {
                map.insert(i, 1);
            }
        }
        let mut ans = String::new();
        let mut count = 0;
        for (k, v) in map {
            if v > count {
                count = v;
                ans = k;
            }
        }
        ans
    }
}

fn main() {
    println!(
        "{}",
        Solution::most_common_word(
            "Bob hit a ball, the hit BALL flew far after it was hit.".to_string(),
            vec!["hit".to_string()]
        )
    );
    println!(
        "{}",
        Solution::most_common_word("a, a, a, a, b,b,b,c, c".to_string(), vec!["a".to_string()])
    );
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::most_common_word(
                "Bob hit a ball, the hit BALL flew far after it was hit.".to_string(),
                vec!["hit".to_string()]
            ),
            "ball"
        );
        assert_eq!(
            Solution::most_common_word("a, a, a, a, b,b,b,c, c".to_string(), vec!["a".to_string()]),
            "b"
        );
    }
}
