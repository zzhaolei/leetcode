struct Solution;

impl Solution {
    pub fn replace_words(dictionary: Vec<String>, sentence: String) -> String {
        let mut ans = String::new();
        for mut i in sentence.trim().split_ascii_whitespace() {
            for j in dictionary.iter() {
                if i.starts_with(j) {
                    i = i.min(j);
                }
            }

            ans.push_str(i);
            ans.push(' ');
        }
        ans.trim().to_string()
    }
}

fn main() {
    println!(
        "{}",
        Solution::replace_words(
            vec!["cat".to_string(), "bat".to_string(), "rat".to_string()],
            "the cattle was rattled by the battery".to_string()
        )
    );
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::replace_words(
                vec![
                    "catt".to_string(),
                    "cat".to_string(),
                    "bat".to_string(),
                    "rat".to_string()
                ],
                "the cattle was rattled by the battery".to_string()
            ),
            "the cat was rat by the bat"
        );
        assert_eq!(
            Solution::replace_words(
                vec!["a".to_string(), "b".to_string(), "c".to_string()],
                "aadsfasf absbs bbab cadsfafs".to_string()
            ),
            "a a b c"
        );
    }
}
