struct Solution;

impl Solution {
    pub fn special_array(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return -1;
        }

        let mut ans = -1;
        for i in 0..=nums.len() {
            let j = i as i32;
            if nums.iter().filter(|x| **x >= j).count() == i {
                ans = j;
            }
        }
        ans
    }
}

fn main() {
    println!("{}", Solution::special_array(vec![]));
    println!("{}", Solution::special_array(vec![0, 0]));
    println!("{}", Solution::special_array(vec![3, 5]));
    println!("{}", Solution::special_array(vec![0, 4, 3, 0, 4]));
    println!("{}", Solution::special_array(vec![3, 6, 7, 7, 0]));
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        assert_eq!(-1, Solution::special_array(vec![]));
        assert_eq!(-1, Solution::special_array(vec![0, 0]));
        assert_eq!(2, Solution::special_array(vec![3, 5]));
        assert_eq!(3, Solution::special_array(vec![0, 4, 3, 0, 4]));
        assert_eq!(-1, Solution::special_array(vec![3, 6, 7, 7, 0]));
    }
}
