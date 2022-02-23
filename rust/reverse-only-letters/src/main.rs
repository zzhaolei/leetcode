struct Solution;

impl Solution {
    pub fn reverse_only_letters(s: String) -> String {
        if s.len() == 1 {
            return s;
        }

        let mut left = 0;
        let mut right = s.len() - 1;
        let mut s = s.chars().collect::<Vec<char>>();
        let letter = |c| ('a'..='z').any(|x| x == c) || ('A'..='Z').any(|x| x == c);
        while left <= right {
            if !letter(s[left]) {
                left += 1;
                continue;
            }
            if !letter(s[right]) {
                right -= 1;
                continue;
            }
            s.swap(left, right);
            left += 1;
            right -= 1;
        }
        s.into_iter().collect::<String>()
    }
}

fn main() {
    println!("{}", Solution::reverse_only_letters("ab-cd".to_string()));
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_impl1() {
        assert_eq!(Solution::reverse_only_letters("a".to_string()), "a");
        assert_eq!(Solution::reverse_only_letters("ab-cd".to_string()), "dc-ba");
        assert_eq!(
            Solution::reverse_only_letters("a-bC-dEf-ghIj".to_string()),
            "j-Ih-gfE-dCba"
        );
        assert_eq!(
            Solution::reverse_only_letters("Test1ng-Leet=code-Q!".to_string()),
            "Qedo1ct-eeLg=ntse-T!"
        );
    }
}
