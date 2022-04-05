struct Solution;

impl Solution {
    fn impl1(left: i32, right: i32) -> i32 {
        let mut count = 0;
        for i in left..=right {
            match format!("{:b}", i)
                .chars()
                .filter(|&x| x == '1')
                .collect::<String>()
                .len()
            {
                2 | 3 | 5 | 7 | 11 | 13 | 17 | 19 => count += 1,
                _ => continue,
            }
        }
        count
    }
    fn impl2(left: i32, right: i32) -> i32 {
        (left..=right).fold(0, |count, x| match x.count_ones() {
            2 | 3 | 5 | 7 | 11 | 13 | 17 | 19 => count + 1,
            _ => count,
        })
    }
    pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
        Solution::impl1(left, right)
    }
}

fn main() {
    println!("{}", Solution::count_prime_set_bits(6, 10));
    println!("{}", Solution::impl1(10, 15));
    println!("{}", Solution::impl2(990, 1048));
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::impl1(6, 10), 4);
        assert_eq!(Solution::impl1(10, 15), 5);
        assert_eq!(Solution::impl1(990, 1048), 28);

        assert_eq!(Solution::impl2(6, 10), 4);
        assert_eq!(Solution::impl2(10, 15), 5);
        assert_eq!(Solution::impl2(990, 1048), 28);
    }
}
