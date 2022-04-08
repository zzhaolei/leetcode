struct Solution;

impl Solution {
    fn impl1(x: i32) -> i32 {
        (x as f64).sqrt() as i32
    }

    #[allow(unused)]
    fn impl2(x: i32) -> i32 {
        if x == 1 || x == 0 {
            return x;
        }
        let mut ans = (x / 2) as i64;
        while ans * ans > x as i64 {
            ans -= 1;
        }
        ans as i32
    }

    pub fn my_sqrt(x: i32) -> i32 {
        Solution::impl1(x)
    }
}

fn main() {
    println!("{}", Solution::my_sqrt(4));
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_impl1() {
        assert_eq!(Solution::impl1(4), 2);
        assert_eq!(Solution::impl1(8), 2);
        assert_eq!(Solution::impl1(2147395599), 46339);
    }

    #[test]
    fn test_impl2() {
        assert_eq!(Solution::impl2(2), 1);
        assert_eq!(Solution::impl2(1), 1);
        assert_eq!(Solution::impl2(0), 0);
        assert_eq!(Solution::impl2(4), 2);
        assert_eq!(Solution::impl2(8), 2);
        assert_eq!(Solution::impl2(2147395599), 46339);
        assert_eq!(Solution::impl2(1978959248), 44485);
    }
}
