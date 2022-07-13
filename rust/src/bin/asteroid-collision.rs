struct Solution;

impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let len = asteroids.len();
        let mut ans: Vec<i32> = Vec::with_capacity(len);
        let mut i = 0;
        while i < len {
            let a = asteroids[i];
            match ans.last() {
                None => ans.push(a),
                Some(&last) => {
                    if last < 0 || a > 0 {
                        ans.push(a);
                        i += 1;
                        continue;
                    }
                    if -a >= last {
                        ans.pop();
                    }
                    if -a > last {
                        continue;
                    }
                }
            }
            i += 1;
        }
        ans
    }
}

fn main() {
    println!("{:?}", Solution::asteroid_collision(vec![5, 10, -5]));
    println!("{:?}", Solution::asteroid_collision(vec![8, -8]));
    println!("{:?}", Solution::asteroid_collision(vec![10, 2, -5]));
    println!("{:?}", Solution::asteroid_collision(vec![-2, -1, 1, 2]));
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::asteroid_collision(vec![5, 10, -5]), [5, 10]);
        assert_eq!(Solution::asteroid_collision(vec![8, -8]), []);
        assert_eq!(Solution::asteroid_collision(vec![10, 2, -5]), [10]);
        assert_eq!(
            Solution::asteroid_collision(vec![-2, -1, 1, 2]),
            [-2, -1, 1, 2]
        );
    }
}
