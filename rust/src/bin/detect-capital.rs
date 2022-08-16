#![allow(unused)]

struct Solution;

impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        word.chars().all(|x| x.is_ascii_lowercase())
            || word.chars().all(|x| x.is_ascii_uppercase())
            || word.char_indices().all(|(i, x)| {
                if i == 0 {
                    x.is_ascii_uppercase()
                } else {
                    x.is_ascii_lowercase()
                }
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_capital_use() {
        assert!(Solution::detect_capital_use(String::from("")));
        assert!(Solution::detect_capital_use(String::from("Zhao")));
        assert!(!Solution::detect_capital_use(String::from("zHao")));
        assert!(Solution::detect_capital_use(String::from("zhao")));
        assert!(Solution::detect_capital_use(String::from("ZHAO")));
    }
}
fn main() {}
