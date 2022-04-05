struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let x = x.to_string();
        x.chars().rev().collect::<String>() == x
    }
}

fn main() {
    println!("{}", Solution::is_palindrome(121));
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        assert!(Solution::is_palindrome(121));
        assert!(!Solution::is_palindrome(-121));
        assert!(!Solution::is_palindrome(123));
    }
}
