struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut ans: i32 = 0;
        let mut prev: Option<char> = None;
        for i in s.chars().rev() {
            let now = prev;
            prev = Some(i);
            match i {
                'I' => {
                    if let Some(n) = now {
                        if n == 'V' || n == 'X' {
                            ans -= 1;
                            continue;
                        }
                    }
                    ans += 1;
                }
                'X' => {
                    if let Some(n) = now {
                        if n == 'L' || n == 'C' {
                            ans -= 10;
                            continue;
                        }
                    }
                    ans += 10;
                }
                'C' => {
                    if let Some(n) = now {
                        if n == 'D' || n == 'M' {
                            ans -= 100;
                            continue;
                        }
                    }
                    ans += 100;
                }
                'V' => ans += 5,
                'L' => ans += 50,
                'D' => ans += 500,
                'M' => ans += 1000,
                _ => {}
            }
        }
        ans
    }
}

fn main() {
    println!("{}", Solution::roman_to_int("III".to_string()));
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::roman_to_int("III".to_string()), 3);
        assert_eq!(Solution::roman_to_int("IV".to_string()), 4);
        assert_eq!(Solution::roman_to_int("IX".to_string()), 9);
        assert_eq!(Solution::roman_to_int("LVIII".to_string()), 58);
        assert_eq!(Solution::roman_to_int("DCXXI".to_string()), 621);
        assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
    }
}
