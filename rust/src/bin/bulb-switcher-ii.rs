struct Solution;

impl Solution {
    pub fn flip_lights(n: i32, presses: i32) -> i32 {
        if presses == 0 {
            return 1;
        }
        if presses == 1 {
            if n < 3 {
                return n + presses;
            }
            return 4;
        }
        if n == 1 {
            return 2;
        }
        if n == 2 {
            return 4;
        }
        if presses < 3 {
            if n < 3 {
                return n + presses;
            }
            return 7;
        }
        8
    }
}

fn main() {
    println!("{}", Solution::flip_lights(100, 3));
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::flip_lights(1000, 3), 8);
    }
}
