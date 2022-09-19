struct Solution;

impl Solution {
    pub fn trim_mean(arr: Vec<i32>) -> f64 {
        let mut arr = arr;
        arr.sort();
        let a = (arr.len() as f64 * 0.05).round() as usize;
        let new = &arr[a..arr.len() - a];
        new.into_iter().sum::<i32>() as f64 / new.len() as f64
    }
}

fn main() {
    println!(
        "{}",
        Solution::trim_mean(vec![
            6, 2, 7, 5, 1, 2, 0, 3, 10, 2, 5, 0, 5, 5, 0, 8, 7, 6, 8, 0
        ])
    );
    println!(
        "{}",
        Solution::trim_mean(vec![
            6, 0, 7, 0, 7, 5, 7, 8, 3, 4, 0, 7, 8, 1, 6, 8, 1, 1, 2, 4, 8, 1, 9, 5, 4, 3, 8, 5, 10,
            8, 6, 6, 1, 0, 6, 10, 8, 2, 3, 4
        ])
    );
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        assert_eq!(
            4.0,
            Solution::trim_mean(vec![
                6, 2, 7, 5, 1, 2, 0, 3, 10, 2, 5, 0, 5, 5, 0, 8, 7, 6, 8, 0
            ])
        );
        assert_eq!(
            4.777777777777778,
            Solution::trim_mean(vec![
                6, 0, 7, 0, 7, 5, 7, 8, 3, 4, 0, 7, 8, 1, 6, 8, 1, 1, 2, 4, 8, 1, 9, 5, 4, 3, 8, 5,
                10, 8, 6, 6, 1, 0, 6, 10, 8, 2, 3, 4
            ])
        );
    }
}
