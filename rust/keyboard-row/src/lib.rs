use std::collections::HashMap;

struct Solution;

impl Solution {
    fn impl1(words: Vec<String>) -> Vec<String> {
        use std::collections::HashSet;

        let one: HashSet<char> = "qwertyuiop".chars().collect();
        let two: HashSet<char> = "asdfghjkl".chars().collect();
        let three: HashSet<char> = "zxcvbnm".chars().collect();

        let find_by_line = |s: &String, line: &HashSet<char>| -> bool {
            for j in s.to_lowercase().chars() {
                if let None = line.get(&j) {
                    return false;
                }
            }
            return true;
        };

        let mut ans = Vec::new();
        for i in words.into_iter() {
            if find_by_line(&i, &one) {
                ans.push(i);
            } else if find_by_line(&i, &two) {
                ans.push(i);
            } else if find_by_line(&i, &three) {
                ans.push(i);
            }
        }

        ans
    }

    fn impl2(words: Vec<String>) -> Vec<String> {
        use std::collections::HashSet;

        let one: HashSet<char> = "qwertyuiop".chars().collect();
        let two: HashSet<char> = "asdfghjkl".chars().collect();
        let three: HashSet<char> = "zxcvbnm".chars().collect();

        let find_by_line = |s: &String, line: &HashSet<char>| -> bool {
            for j in s.to_lowercase().chars() {
                if let None = line.get(&j) {
                    return false;
                }
            }
            return true;
        };

        return words
            .into_iter()
            .filter(|s| -> bool {
                if find_by_line(s, &one) {
                    true
                } else if find_by_line(s, &two) {
                    true
                } else if find_by_line(s, &three) {
                    true
                } else {
                    false
                }
            })
            .collect();
    }

    fn init_map() -> HashMap<char, u8> {
        let mut map = HashMap::new();
        for (i, s) in ["qwertyuiop", "asdfghjkl", "zxcvbnm"]
            .into_iter()
            .enumerate()
        {
            map = s.chars().fold(map, |mut map, c| {
                map.insert(c, i as u8);
                map
            });
        }
        map
    }

    fn impl3(words: Vec<String>) -> Vec<String> {
        let map = Solution::init_map();
        return words
            .into_iter()
            .filter(|s| -> bool {
                let mut n = None;
                for i in s.to_lowercase().chars() {
                    let v = map.get(&i).unwrap();
                    if let Some(vv) = n {
                        if vv != v {
                            return false;
                        }
                    } else {
                        n = Some(v);
                    }
                }
                return true;
            })
            .collect();
    }

    fn impl4(words: Vec<String>) -> Vec<String> {
        use std::collections::HashSet;

        let one: HashSet<char> = "qwertyuiop".chars().collect();
        let two: HashSet<char> = "asdfghjkl".chars().collect();
        let three: HashSet<char> = "zxcvbnm".chars().collect();

        let find_by_line = |s: &String| -> bool {
            let s = s.to_lowercase().chars().collect::<HashSet<char>>();
            let s_len = s.len();

            let mut len = s.intersection(&one).collect::<Vec<&char>>().len();
            if len == s_len {
                return true;
            } else if len != 0 {
                return false;
            }

            len = s.intersection(&two).collect::<Vec<&char>>().len();
            if len == s_len {
                return true;
            } else if len != 0 {
                return false;
            }

            len = s.intersection(&three).collect::<Vec<&char>>().len();
            if len == s_len {
                return true;
            } else if len != 0 {
                return false;
            }

            false
        };

        return words
            .into_iter()
            .filter(|s| -> bool { find_by_line(s) })
            .collect();
    }

    pub fn find_words(words: Vec<String>) -> Vec<String> {
        Solution::impl1(words)
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn impl1() {
        let v = vec![
            "Hello".to_string(),
            "Alaska".to_string(),
            "Dad".to_string(),
            "Peace".to_string(),
        ];
        let r = Solution::impl1(v);
        assert_eq!(r, vec!["Alaska", "Dad"]);
    }

    #[test]
    fn impl2() {
        let v = vec![
            "Hello".to_string(),
            "Alaska".to_string(),
            "Dad".to_string(),
            "Peace".to_string(),
        ];
        let r = Solution::impl2(v);
        assert_eq!(r, vec!["Alaska", "Dad"]);
    }

    #[test]
    fn impl3() {
        let v = vec![
            "Hello".to_string(),
            "Alaska".to_string(),
            "Dad".to_string(),
            "Peace".to_string(),
        ];
        let r = Solution::impl3(v);
        assert_eq!(r, vec!["Alaska", "Dad"]);

        let v = vec!["a".to_string(), "b".to_string()];
        let r = Solution::impl3(v);
        assert_eq!(r, vec!["a", "b"]);
    }

    #[test]
    fn impl4() {
        let v = vec![
            "Hello".to_string(),
            "Alaska".to_string(),
            "Dad".to_string(),
            "Peace".to_string(),
        ];
        let r = Solution::impl4(v);
        assert_eq!(r, vec!["Alaska", "Dad"]);

        let v = vec!["a".to_string(), "b".to_string()];
        let r = Solution::impl3(v);
        assert_eq!(r, vec!["a", "b"]);
    }
}
