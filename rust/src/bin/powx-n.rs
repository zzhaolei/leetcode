struct Solution;

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if n >= 0 {
            Solution::quick_mul(x, n)
        } else {
            1.0 / Solution::quick_mul(x, -n)
        }
    }

    pub fn quick_mul(x: f64, n: i32) -> f64 {
        if n == 0 {
            return 1.0;
        }
        let y = Solution::quick_mul(x, n / 2);
        if n % 2 == 0 {
            y * y
        } else {
            y * y * x
        }
    }
}

fn main() {
    println!("{}", Solution::my_pow(2.00000, 10));
    println!("{}", Solution::my_pow(2.10000, 3));
    println!("{}", Solution::my_pow(2.00000, -2));
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::my_pow(2.00000, 10), 1024.0);
        assert_eq!(Solution::my_pow(2.10000, 3), 9.261000000000001);
        assert_eq!(Solution::my_pow(2.00000, -2), 0.25);
    }
}
