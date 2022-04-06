struct Solution;

impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        let delete = |s: String| -> String {
            let mut ans = String::new();
            let mut count = 0;
            for i in s.chars().rev() {
                if i == '#' {
                    count += 1;
                } else {
                    if count != 0 {
                        count -= 1;
                    } else {
                        ans.push(i);
                    }
                }
            }
            ans
        };
        delete(s) == delete(t)
    }
}

fn main() {
    println!(
        "{}",
        Solution::backspace_compare("ab#c".to_string(), "ad#c".to_string())
    );
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        assert!(Solution::backspace_compare(
            "ab#c".to_string(),
            "ad#c".to_string()
        ));
        assert!(Solution::backspace_compare(
            "ab##".to_string(),
            "c#d#".to_string()
        ));
        assert!(!Solution::backspace_compare(
            "a#c".to_string(),
            "b".to_string()
        ));
    }
}
