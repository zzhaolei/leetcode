struct Solution;

impl Solution {
    pub fn busy_student(start_time: Vec<i32>, end_time: Vec<i32>, query_time: i32) -> i32 {
        (0..start_time.len())
            .filter(|&i| query_time >= start_time[i] && query_time <= end_time[i])
            .count() as i32
    }
}

fn main() {
    println!(
        "{}",
        Solution::busy_student(vec![1, 2, 3], vec![3, 2, 7], 4)
    );
    println!("{}", Solution::busy_student(vec![4], vec![4], 4));
    println!("{}", Solution::busy_student(vec![4], vec![4], 5));
    println!(
        "{}",
        Solution::busy_student(vec![1, 1, 1, 1], vec![1, 3, 2, 4], 7)
    );
    println!(
        "{}",
        Solution::busy_student(
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1],
            vec![10, 10, 10, 10, 10, 10, 10, 10, 10],
            5
        )
    );
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        assert_eq!(1, Solution::busy_student(vec![1, 2, 3], vec![3, 2, 7], 4));
        assert_eq!(1, Solution::busy_student(vec![4], vec![4], 4));
        assert_eq!(0, Solution::busy_student(vec![4], vec![4], 5));
        assert_eq!(
            0,
            Solution::busy_student(vec![1, 1, 1, 1], vec![1, 3, 2, 4], 7)
        );
        assert_eq!(
            5,
            Solution::busy_student(
                vec![9, 8, 7, 6, 5, 4, 3, 2, 1],
                vec![10, 10, 10, 10, 10, 10, 10, 10, 10],
                5
            )
        );
    }
}
