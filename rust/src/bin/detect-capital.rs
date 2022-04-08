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
        assert_eq!(Solution::detect_capital_use(String::from("")), true);
        assert_eq!(Solution::detect_capital_use(String::from("Zhao")), true);
        assert_eq!(Solution::detect_capital_use(String::from("zHao")), false);
        assert_eq!(Solution::detect_capital_use(String::from("zhao")), true);
        assert_eq!(Solution::detect_capital_use(String::from("ZHAO")), true);
    }
}
fn main() {}
