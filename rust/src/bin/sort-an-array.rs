struct Solution;

impl Solution {
    pub fn sort_array(mut vec: Vec<i32>) -> Vec<i32> {
        vec.sort_unstable();
        vec
    }
}

fn main() {
    println!("{:?}", Solution::sort_array(vec![5, 2, 3, 1]));
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::sort_array(vec![5, 2, 3, 1]), [1, 2, 3, 5]);
    }
}
