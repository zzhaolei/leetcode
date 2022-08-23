struct Solution;

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        use std::collections::HashMap;

        let s = s.split_ascii_whitespace().collect::<Vec<&str>>();
        let pattern = pattern.chars().collect::<Vec<char>>();
        if s.len() != pattern.len() {
            return false;
        }
        let mut map: HashMap<char, &str> = HashMap::with_capacity(pattern.len());
        let mut map_r: HashMap<&str, char> = HashMap::with_capacity(pattern.len());
        for i in 0..pattern.len() {
            let key = pattern[i];
            let value = s[i];
            if let Some(&v) = map.get(&key) {
                if v != value {
                    return false;
                }
            } else if let Some(&v) = map_r.get(value) {
                if v != key {
                    return false;
                }
            }
            map.insert(key, value);
            map_r.insert(value, key);
        }
        true
    }
}

fn main() {
    println!(
        "true = {}",
        Solution::word_pattern("abba".into(), "dog cat cat dog".into())
    );
    println!(
        "true = {}",
        Solution::word_pattern("abc".into(), "b c a".into())
    );
    println!(
        "false = {}",
        Solution::word_pattern("abba".into(), "dog cat cat fish".into())
    );
    println!(
        "false = {}",
        Solution::word_pattern("aaaa".into(), "dog cat cat dog".into())
    );
    println!(
        "false = {}",
        Solution::word_pattern("abba".into(), "dog dog dog dog".into())
    );
}
