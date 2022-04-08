struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut ans: Vec<char> = vec![' '];
        for i in s.chars() {
            match i {
                '(' | '[' | '{' => ans.push(i),
                ')' => {
                    if ans.pop().unwrap() != '(' {
                        return false;
                    }
                }
                ']' => {
                    if ans.pop().unwrap() != '[' {
                        return false;
                    }
                }
                '}' => {
                    if ans.pop().unwrap() != '{' {
                        return false;
                    }
                }
                _ => return false,
            }
        }
        ans.len() == 1
    }
}

fn main() {
    println!("{}", Solution::is_valid("{[()]}".to_string()));
    println!("{}", Solution::is_valid("]{[()]}".to_string()));
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        assert!(Solution::is_valid("{[()]}".to_string()));
        assert!(!Solution::is_valid("]{[()]}".to_string()));
    }
}
