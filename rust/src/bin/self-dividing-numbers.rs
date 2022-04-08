struct Solution;

impl Solution {
    fn is_self_dividing_number(n: i32) -> bool {
        let mut b = n;
        let mut a;
        loop {
            a = b % 10;
            if a == 0 || n % a != 0 {
                return false;
            }
            b /= 10;
            if b == 0 {
                break;
            }
        }
        true
    }

    pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
        let mut ans = Vec::new();
        for i in left..=right {
            if i != 0 && Solution::is_self_dividing_number(i) {
                ans.push(i);
            }
        }
        ans
    }
}

fn main() {
    println!("{:?}", Solution::self_dividing_numbers(1, 22));
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::self_dividing_numbers(1, 22),
            [1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 15, 22]
        );
        assert_eq!(Solution::self_dividing_numbers(47, 85), [48, 55, 66, 77]);
    }
}
