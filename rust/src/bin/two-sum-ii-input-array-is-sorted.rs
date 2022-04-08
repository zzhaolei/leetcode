use std::cmp::Ordering;

struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let (mut i, mut j) = (0, numbers.len() - 1);
        let mut ans;
        loop {
            if i >= j {
                break vec![];
            }
            ans = numbers[i] + numbers[j];
            match ans.cmp(&target) {
                Ordering::Equal => break vec![(i + 1) as i32, (j + 1) as i32],
                Ordering::Less => i += 1,
                Ordering::Greater => j -= 1,
            }
        }
    }
}

fn main() {
    println!("{:?}", Solution::two_sum(vec![2, 7, 11, 15], 9));
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), [1, 2]);
        assert_eq!(Solution::two_sum(vec![2, 3, 4], 6), [1, 3]);
        assert_eq!(Solution::two_sum(vec![-1, 0], -1), [1, 2]);
    }
}
