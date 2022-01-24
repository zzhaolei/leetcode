struct Solution;

impl Solution {
    pub fn impl1(mut nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        nums1.extend(nums2);
        nums1.sort_unstable();
        let len = nums1.len();
        if len % 2 == 0 {
            (nums1[len / 2] + nums1[len / 2 - 1]) as f64 / 2.0
        } else {
            nums1[len / 2] as f64
        }
    }

    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        Solution::impl1(nums1, nums2)
    }
}

fn main() {
    println!(
        "{}",
        Solution::find_median_sorted_arrays(vec![1, 3], vec![2])
    );
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_impl1() {
        assert_eq!(Solution::impl1(vec![1, 3], vec![2]), 2.0);
        assert_eq!(Solution::impl1(vec![1, 2], vec![3, 4]), 2.5);
        assert_eq!(Solution::impl1(vec![0, 0], vec![0, 0]), 0.0);
        assert_eq!(Solution::impl1(vec![], vec![1]), 1.0);
        assert_eq!(Solution::impl1(vec![2], vec![]), 2.0);
    }
}
