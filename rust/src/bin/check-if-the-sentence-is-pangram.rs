struct Solution;

impl Solution {
    pub fn check_if_pangram(sentence: String) -> bool {
        use std::collections::HashSet;

        sentence.chars().collect::<HashSet<char>>().len() >= 26
    }
}

fn main() {
    println!(
        "{}",
        Solution::check_if_pangram("thequickbrownfoxjumpsoverthelazydog".to_string())
    );
    println!("{}", Solution::check_if_pangram("leetcode".to_string()));
}
