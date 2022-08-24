struct Solution;

impl Solution {
    pub fn can_be_equal(target: Vec<i32>, arr: Vec<i32>) -> bool {
        let mut t = target;
        t.sort();
        let mut a = arr;
        a.sort();
        t == a
    }
}

fn main() {
    println!(
        "{}",
        Solution::can_be_equal(vec![1, 2, 3, 4], vec![2, 4, 1, 3])
    );
    println!("{}", Solution::can_be_equal(vec![7], vec![7]));
    println!("{}", Solution::can_be_equal(vec![3, 7, 9], vec![3, 7, 11]));
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        assert!(Solution::can_be_equal(vec![1, 2, 3, 4], vec![2, 4, 1, 3]));
        assert!(Solution::can_be_equal(vec![7], vec![7]));
        assert!(!Solution::can_be_equal(vec![3, 7, 9], vec![3, 7, 11]));
    }
}
