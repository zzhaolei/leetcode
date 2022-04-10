struct Solution;

impl Solution {
    pub fn unique_morse_representations(words: Vec<String>) -> i32 {
        use std::collections::HashSet;

        let morse: [&str; 26] = [
            ".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..",
            "--", "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-",
            "-.--", "--..",
        ];
        let mut ans: HashSet<String> = HashSet::new();
        for word in words.iter() {
            let mut morse_string = String::new();
            for c in word.chars() {
                // a == 97
                morse_string.push_str(morse[c as usize - 97]);
            }
            ans.insert(morse_string);
        }
        ans.len() as i32
    }
}

fn main() {
    println!(
        "{}",
        Solution::unique_morse_representations(vec![
            "gin".to_string(),
            "zen".to_string(),
            "gig".to_string(),
            "msg".to_string()
        ])
    )
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::unique_morse_representations(vec![
                "gin".to_string(),
                "zen".to_string(),
                "gig".to_string(),
                "msg".to_string()
            ]),
            2
        );
        assert_eq!(
            Solution::unique_morse_representations(vec!["a".to_string()]),
            1
        );
    }
}
