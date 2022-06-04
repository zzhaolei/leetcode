struct Solution;

impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        use std::collections::HashSet;
        emails
            .into_iter()
            .map(|email| {
                let (mut at, mut plus) = (false, false);
                email
                    .chars()
                    .filter(|c| match c {
                        '@' => {
                            at = true;
                            plus = false;
                            true
                        }
                        '+' if !at => {
                            plus = true;
                            false
                        }
                        '.' if !at => false,
                        _ if plus => false,
                        _ => true,
                    })
                    .collect::<String>()
            })
            .collect::<HashSet<String>>()
            .len() as i32
    }
}

fn main() {
    println!(
        "{}",
        Solution::num_unique_emails(vec![
            "test.email+alex@leetcode.com".to_string(),
            "test.e.mail+bob.cathy@leetcode.com".to_string(),
            "testemail+david@lee.tcode.com".to_string()
        ])
    )
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::num_unique_emails(vec![
                "test.email+alex@leetcode.com".to_string(),
                "test.e.mail+bob.cathy@leetcode.com".to_string(),
                "testemail+david@lee.tcode.com".to_string()
            ]),
            2
        );
        assert_eq!(
            Solution::num_unique_emails(vec![
                "a@leetcode.com".to_string(),
                "b@leetcode.com".to_string(),
                "c@leetcode.com".to_string()
            ]),
            3
        );
    }
}
