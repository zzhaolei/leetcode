struct Solution;

impl Solution {
    pub fn impl1(x: i32) -> i32 {
        let mut s = x.to_string();
        let ans;
        if s.starts_with('-') {
            s = (&s[1..]).chars().rev().collect::<String>();
            ans = -s.parse::<i64>().unwrap_or(0);
        } else {
            s = (&s[..]).chars().rev().collect::<String>();
            ans = s.parse::<i64>().unwrap_or(0);
        }

        if ans > i32::MAX as i64 || ans < i32::MIN as i64 {
            return 0;
        }
        ans as i32
    }

    pub fn impl2(x: i32) -> i32 {
        if x == 0 {
            return x;
        }
        let mut x = x as i64;
        let mut ans: i64 = 0;
        while x != 0 {
            let m = x % 10;
            x /= 10;
            ans = ans * 10 + m;
        }

        if ans > i32::MAX as i64 || ans < i32::MIN as i64 {
            0
        } else {
            ans as i32
        }
    }

    pub fn reverse(x: i32) -> i32 {
        Solution::impl1(x)
    }
}

fn main() {
    println!("{}", Solution::reverse(-123));
    println!("{}", Solution::impl2(-123));
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_impl1() {
        assert_eq!(Solution::impl1(123), 321);
        assert_eq!(Solution::impl1(-123), -321);
    }

    #[test]
    fn test_impl2() {
        assert_eq!(Solution::impl2(123), 321);
        assert_eq!(Solution::impl2(-123), -321);
        assert_eq!(Solution::impl2(1534236469), 0);
    }
}
