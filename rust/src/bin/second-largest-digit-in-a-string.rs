struct Solution;

impl Solution {
    pub fn second_highest(s: String) -> i32 {
        let (mut a, mut b) = (47, 47);
        s.chars().for_each(|c| {
            if ('0'..='9').contains(&c) {
                let i = c as i32;
                if i > b {
                    if b > a {
                        a = b;
                    }
                    b = i;
                }
                if i < b && i > a {
                    a = i;
                }
            }
        });
        a - 48
    }
}

fn main() {
    println!("{}", Solution::second_highest("dfa12321afd".to_string()));
    println!("{}", Solution::second_highest("abc1111".to_string()));
    println!("{}", Solution::second_highest("abc2111".to_string()));
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::second_highest("dfa12321afd".to_string()), 2);
        assert_eq!(Solution::second_highest("abc1111".to_string()), -1);
        assert_eq!(Solution::second_highest("abc2111".to_string()), 1);
    }
}
