struct Solution;

impl Solution {
    pub fn is_covered(ranges: Vec<Vec<i32>>, left: i32, right: i32) -> bool {
        let mut ranges = ranges;
        let mut left = left;
        ranges.sort_by(|i, j| i[0].cmp(&j[0]));
        for i in ranges.into_iter() {
            if left >= i[0] && left <= i[1] {
                left = i[1] + 1;
            }
        }
        left > right
    }
}

fn main() {
    println!(
        "{}",
        Solution::is_covered(vec![vec![1, 2], vec![3, 4], vec![5, 6]], 2, 5)
    );
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::is_covered(vec![vec![1, 2], vec![3, 4], vec![5, 6]], 2, 5),
            true
        );
        assert_eq!(
            Solution::is_covered(vec![vec![1, 10], vec![10, 20]], 21, 21),
            false
        );
    }
}
