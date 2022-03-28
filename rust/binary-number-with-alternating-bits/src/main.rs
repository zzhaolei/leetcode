struct Solution;

impl Solution {
    fn impl1(n: i32) -> bool {
        if n == 1 || n == 0 {
            return true;
        }
        let mut p = n % 2;
        let mut n = n / 2;
        while n >= 1 {
            let t = n % 2;
            if t == p {
                return false;
            }
            n /= 2;
            p = t;
        }
        true
    }

    fn impl2(n: i32) -> bool {
        let a = n ^ (n >> 1);
        a & (a + 1) == 0
    }

    pub fn has_alternating_bits(n: i32) -> bool {
        Solution::impl1(n)
    }
}

fn main() {
    println!("{}", Solution::has_alternating_bits(3));
    println!("{}", Solution::impl2(3));
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        assert!(Solution::has_alternating_bits(5));
        assert!(!Solution::has_alternating_bits(7));
    }
}
