struct Solution;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        use std::collections::HashMap;

        let mut s_m: HashMap<char, char> = HashMap::new();
        let mut t_m: HashMap<char, char> = HashMap::new();
        let t = t.chars().collect::<Vec<char>>();

        for (i, c) in s.chars().enumerate() {
            let t_c = t[i];
            if let Some(&v) = s_m.get(&c) {
                if v != t_c {
                    return false;
                }
            } else if let Some(&v) = t_m.get(&t_c) {
                if v != c {
                    return false;
                }
            } else {
                s_m.insert(c, t_c);
                t_m.insert(t_c, c);
            }
        }
        true
    }
}

fn main() {
    let tests = vec![
        ("bbbaaaba", "aaabbbba"),
        ("egg", "add"),
        ("paper", "title"),
        ("a", "a"),
        ("aaeaa", "uuxyy"),
        ("badc", "baba"),
    ];
    for (s, t) in tests {
        println!(
            "{s}, {t}: {}",
            Solution::is_isomorphic(s.to_string(), t.to_string())
        );
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        let tests = vec![
            ("bbbaaaba", "aaabbbba", false),
            ("egg", "add", true),
            ("paper", "title", true),
            ("a", "a", true),
            ("aaeaa", "uuxyy", false),
            ("badc", "baba", false),
        ];
        for (s, t, r) in tests {
            assert_eq!(Solution::is_isomorphic(s.to_string(), t.to_string()), r);
        }
    }
}
