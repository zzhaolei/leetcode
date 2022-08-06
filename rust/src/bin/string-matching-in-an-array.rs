struct Solution;

impl Solution {
    pub fn string_matching(words: Vec<String>) -> Vec<String> {
        use std::collections::HashSet;

        let mut ans = HashSet::new();
        for (i, v) in words.iter().enumerate() {
            for j in words.iter().skip(i + 1) {
                if v.contains(j) {
                    ans.insert(j.clone());
                } else if j.contains(v) {
                    ans.insert(v.clone());
                }
            }
        }
        ans.into_iter().collect::<Vec<String>>()
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::string_matching(vec![
            "mass".to_string(),
            "as".to_string(),
            "hero".to_string(),
            "superhero".to_string()
        ])
    );
    println!(
        "{:?}",
        Solution::string_matching(vec![
            "leetcode".to_string(),
            "et".to_string(),
            "code".to_string(),
        ])
    );
    println!(
        "{:?}",
        Solution::string_matching(vec![
            "blue".to_string(),
            "green".to_string(),
            "bu".to_string(),
        ])
    );
}
