struct Solution;

impl Solution {
    fn impl1(n: i32) -> i32 {
        if n <= 1 {
            n
        } else {
            Solution::fib(n - 1) + Solution::fib(n - 2)
        }
    }

    #[allow(unused)]
    fn impl2(n: i32) -> i32 {
        (0..n).fold((0, 1), |t, _| (t.1, t.0 + t.1)).0
    }

    pub fn fib(n: i32) -> i32 {
        Solution::impl1(n)
    }
}

fn main() {
    println!("{}", Solution::fib(2));
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_impl1() {
        assert_eq!(Solution::impl1(2), 1);
        assert_eq!(Solution::impl1(3), 2);
        assert_eq!(Solution::impl1(4), 3);
    }

    #[test]
    fn test_impl2() {
        assert_eq!(Solution::impl2(2), 1);
        assert_eq!(Solution::impl2(3), 2);
        assert_eq!(Solution::impl2(4), 3);
    }
}
