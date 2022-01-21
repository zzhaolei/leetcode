struct Solution;

impl Solution {
    pub fn impl1(s: String) -> i32 {
        let mut tmp = String::new();
        let mut ans = 0i64;
        let i32_max = i32::MAX as i64;
        let i32_min = i32::MIN as i64;
        for (i, c) in s.trim_matches(' ').chars().enumerate() {
            match c {
                '-' | '+' if i == 0 => {
                    tmp.push(c);
                    continue;
                }
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    tmp.push(c);
                }
                _ => return ans as i32,
            }
            ans = tmp.parse::<i64>().unwrap_or(0);
            if ans > i32_max {
                return i32::MAX;
            } else if ans < i32_min {
                return i32::MIN;
            }
        }
        ans as i32
    }

    pub fn my_atoi(s: String) -> i32 {
        Solution::impl1(s)
    }
}

fn main() {
    println!(
        "{}",
        Solution::my_atoi("21474836471 with words".to_string())
    );
    println!(
        "{}",
        Solution::my_atoi("20000000000000000000 with words".to_string())
    );
    println!("i32::MAX: {}, i64::MAX: {}", i32::MAX, i64::MAX);
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_impl1() {
        assert_eq!(
            Solution::my_atoi("21474836471 with words".to_string()),
            i32::MAX
        );
        assert_eq!(
            Solution::my_atoi("20000000000000000000".to_string()),
            i32::MAX
        );
        assert_eq!(
            Solution::my_atoi("1234567890123456789012345678901234567890".to_string()),
            i32::MAX
        );
        assert_eq!(
            Solution::my_atoi("-1234567890123456789012345678901234567890".to_string()),
            i32::MIN
        );
        assert_eq!(
            Solution::my_atoi("  0000000000012345678".to_string()),
            12345678
        );
        assert_eq!(
            Solution::my_atoi("  -0000000000012345678".to_string()),
            -12345678
        );
        assert_eq!(Solution::my_atoi("-00000000000".to_string()), 0);
        assert_eq!(Solution::my_atoi("00000000000".to_string()), 0);
        assert_eq!(Solution::my_atoi("-42 with words".to_string()), -42);
        assert_eq!(Solution::my_atoi("+42 with words".to_string()), 42);
        assert_eq!(Solution::my_atoi(" +004200 with words".to_string()), 4200);
        assert_eq!(Solution::my_atoi(" -004200 with words".to_string()), -4200);
        assert_eq!(Solution::my_atoi("-5-".to_string()), -5);
    }
}
