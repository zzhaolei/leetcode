struct Solution;

impl Solution {
    fn impl1(nums: Vec<i32>) -> i32 {
        for i in (0..nums.len() - 1).step_by(2) {
            if nums[i] != nums[i + 1] {
                return nums[i];
            }
        }
        nums[nums.len() - 1]
    }

    #[allow(unused)]
    fn impl2(nums: Vec<i32>) -> i32 {
        let mut low = 0;
        let mut high = nums.len() - 1;

        while low < high {
            let mid = (low + high) / 2;
            if nums[mid] == nums[mid ^ 1] {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
        nums[low]
    }

    #[allow(unused)]
    fn impl3(nums: Vec<i32>) -> i32 {
        let mut low = 0;
        let mut high = nums.len() - 1;

        while low < high {
            let mut mid = (low + high) / 2;
            mid -= mid & 1;
            if nums[mid] == nums[mid + 1] {
                low = mid + 2;
            } else {
                high = mid;
            }
        }
        nums[low]
    }

    #[allow(unused)]
    fn impl4(nums: Vec<i32>) -> i32 {
        nums.iter().fold(0, |s, x| s ^ x)
    }

    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        Solution::impl1(nums)
    }
}

fn main() {
    println!(
        "{}",
        Solution::single_non_duplicate(vec![1, 1, 2, 3, 3, 4, 4, 8, 8])
    );
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_impl1() {
        assert_eq!(Solution::impl1(vec![3, 3, 7, 7, 10, 11, 11]), 10);
        assert_eq!(Solution::impl1(vec![1, 1, 2, 3, 3, 4, 4, 8, 8]), 2);
        assert_eq!(Solution::impl1(vec![1, 2, 2, 3, 3, 4, 4, 8, 8]), 1);
        assert_eq!(Solution::impl1(vec![1, 1, 2, 2, 3, 3, 4, 4, 8]), 8);
        assert_eq!(Solution::impl1(vec![1, 1, 2, 2, 3, 3, 4, 8, 8]), 4);
    }

    #[test]
    fn test_impl2() {
        assert_eq!(Solution::impl2(vec![3, 3, 7, 7, 10, 11, 11]), 10);
        assert_eq!(Solution::impl2(vec![1, 1, 2, 3, 3, 4, 4, 8, 8]), 2);
        assert_eq!(Solution::impl2(vec![1, 2, 2, 3, 3, 4, 4, 8, 8]), 1);
        assert_eq!(Solution::impl2(vec![1, 1, 2, 2, 3, 3, 4, 4, 8]), 8);
        assert_eq!(Solution::impl2(vec![1, 1, 2, 2, 3, 3, 4, 8, 8]), 4);
    }

    #[test]
    fn test_impl3() {
        assert_eq!(Solution::impl3(vec![3, 3, 7, 7, 10, 11, 11]), 10);
        assert_eq!(Solution::impl3(vec![1, 1, 2, 3, 3, 4, 4, 8, 8]), 2);
        assert_eq!(Solution::impl3(vec![1, 2, 2, 3, 3, 4, 4, 8, 8]), 1);
        assert_eq!(Solution::impl3(vec![1, 1, 2, 2, 3, 3, 4, 4, 8]), 8);
        assert_eq!(Solution::impl3(vec![1, 1, 2, 2, 3, 3, 4, 8, 8]), 4);
    }

    #[test]
    fn test_impl4() {
        assert_eq!(Solution::impl4(vec![3, 3, 7, 7, 10, 11, 11]), 10);
        assert_eq!(Solution::impl4(vec![1, 1, 2, 3, 3, 4, 4, 8, 8]), 2);
        assert_eq!(Solution::impl4(vec![1, 2, 2, 3, 3, 4, 4, 8, 8]), 1);
        assert_eq!(Solution::impl4(vec![1, 1, 2, 2, 3, 3, 4, 4, 8]), 8);
        assert_eq!(Solution::impl4(vec![1, 1, 2, 2, 3, 3, 4, 8, 8]), 4);
    }
}
