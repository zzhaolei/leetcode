struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let n = n as usize;
        let mut v = vec![0; n + 1];
        v[0] = 1;
        v[1] = 1;
        for i in 2..=n {
            v[i] = v[i - 1] + v[i - 2];
        }
        v[n]
    }
}

fn main() {
    println!("{}", Solution::climb_stairs(1));
    println!("{}", Solution::climb_stairs(2));
    println!("{}", Solution::climb_stairs(45));
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::climb_stairs(1), 1);
        assert_eq!(Solution::climb_stairs(2), 2);
        assert_eq!(Solution::climb_stairs(3), 3);
        assert_eq!(Solution::climb_stairs(45), 1836311903);
    }
}
