struct Solution;

impl Solution {
    fn impl1(nums: &mut Vec<i32>, k: i32) {
        for _ in 0..k {
            let p = nums.pop().unwrap();
            nums.insert(0, p);
        }
    }

    #[allow(unused)]
    fn impl2(nums: &mut Vec<i32>, k: i32) {
        let i = nums.len() as i32;
        nums.rotate_right(k.rem_euclid(i) as usize);
    }

    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        Solution::impl1(nums, k)
    }
}

fn main() {
    let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
    Solution::rotate(&mut nums, 3);
    println!("{:?}", nums);

    let mut nums = vec![-1, -100, 3, 99];
    Solution::rotate(&mut nums, 2);
    println!("{:?}", nums);
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_impl1() {
        let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
        Solution::impl1(&mut nums, 3);
        assert_eq!(nums, vec![5, 6, 7, 1, 2, 3, 4]);

        let mut nums = vec![-1, -100, 3, 99];
        Solution::impl1(&mut nums, 2);
        assert_eq!(nums, vec![3, 99, -1, -100]);

        let mut nums = vec![1, 2];
        Solution::impl1(&mut nums, 3);
        assert_eq!(nums, vec![2, 1]);
    }

    #[test]
    fn test_impl2() {
        let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
        Solution::impl2(&mut nums, 3);
        assert_eq!(nums, vec![5, 6, 7, 1, 2, 3, 4]);

        let mut nums = vec![-1, -100, 3, 99];
        Solution::impl2(&mut nums, 2);
        assert_eq!(nums, vec![3, 99, -1, -100]);

        let mut nums = vec![1, 2];
        Solution::impl2(&mut nums, 3);
        assert_eq!(nums, vec![2, 1]);
    }
}
