struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.len() == 1 {
            return strs[0].to_owned();
        }

        let mut ans = String::new();
        let f = strs.first().unwrap().chars();
        for (i, c) in f.enumerate() {
            for cc in strs.iter().skip(1) {
                // 任何不匹配的方式都直接中断
                if let Some(k) = cc.chars().nth(i) {
                    if c != k {
                        return ans;
                    }
                } else {
                    return ans;
                }
            }
            ans.push(c);
        }
        ans
    }
}

fn main() {
    println!(
        "{}",
        Solution::longest_common_prefix(vec!["ab".to_string(), "a".to_string()])
    );
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::longest_common_prefix(vec!["ab".to_string(), "a".to_string()]),
            "a"
        );
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "dog".to_string(),
                "racecar".to_string(),
                "car".to_string()
            ]),
            ""
        );
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "flower".to_string(),
                "flow".to_string(),
                "floght".to_string()
            ]),
            "flo"
        );
    }
}
