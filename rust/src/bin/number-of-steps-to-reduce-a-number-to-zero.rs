struct Solution;

impl Solution {
    pub fn number_of_steps(mut num: i32) -> i32 {
        let mut ans = 0;
        while num > 0 {
            if num % 2 == 0 {
                ans += 1;
                num /= 2;
            } else {
                num -= 1;
                if num == 0 {
                    ans += 1;
                    break;
                }
                num /= 2;
                ans += 2;
            }
        }
        ans
    }
}

fn main() {
    println!("{}", Solution::number_of_steps(8));
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_impl1() {
        assert_eq!(Solution::number_of_steps(14), 6);
        assert_eq!(Solution::number_of_steps(8), 4);
        assert_eq!(Solution::number_of_steps(123), 12);
    }
}
