#![allow(unused)]
struct Solution;

impl Solution {
    pub fn max_count(mut m: i32, mut n: i32, mut ops: Vec<Vec<i32>>) -> i32 {
        ops = ops
            .into_iter()
            .map(|x| {
                m = m.min(x[0]);
                n = n.min(x[1]);
                x
            })
            .collect::<Vec<_>>();
        m * n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_count() {
        assert_eq!(Solution::max_count(3, 3, vec![vec![2, 2], vec![3, 3]]), 4);
        assert_eq!(Solution::max_count(3, 3, vec![]), 9);
        assert_eq!(
            Solution::max_count(
                26,
                17,
                vec![
                    vec![20, 10],
                    vec![26, 11],
                    vec![2, 11],
                    vec![4, 16],
                    vec![2, 3],
                    vec![23, 13],
                    vec![7, 15],
                    vec![11, 11],
                    vec![25, 13],
                    vec![11, 13],
                    vec![13, 11],
                    vec![13, 16],
                    vec![26, 17]
                ]
            ),
            6
        );
    }
}
