struct Solution;

impl Solution {
    pub fn one_edit_away(first: String, second: String) -> bool {
        if first == second {
            return true;
        }
        let mut num = 0;
        let (mut f, mut s) = (
            first.chars().collect::<Vec<char>>(),
            second.chars().collect::<Vec<char>>(),
        );
        if f.len() == s.len() {
            for i in 0..f.len() {
                if f[i] != s[i] {
                    num += 1;
                }
                if num >= 2 {
                    return false;
                }
            }
            true
        } else {
            if f.len() < s.len() {
                std::mem::swap(&mut f, &mut s);
            }
            for i in 0..f.len() {
                let mut ans: Vec<char> = vec![];
                ans.extend(&f[..i]);
                ans.extend(&f[i + 1..]);

                if ans == s {
                    return true;
                }
            }
            false
        }
    }
}

fn main() {
    println!(
        "{}",
        Solution::one_edit_away("pale".to_string(), "ple".to_string())
    );
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        assert!(Solution::one_edit_away(
            "pale".to_string(),
            "ple".to_string()
        ));
        assert!(!Solution::one_edit_away(
            "pales".to_string(),
            "pal".to_string()
        ));
    }
}
