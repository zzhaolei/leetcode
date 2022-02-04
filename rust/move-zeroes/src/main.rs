struct Solution;

impl Solution {
    pub fn impl1(nums: &mut Vec<i32>) {
        let mut zero = vec![];
        for (i, &v) in nums.iter().enumerate() {
            if v == 0 {
                zero.push(i);
            }
        }
        let len = zero.len();
        for (i, &v) in zero.iter().enumerate() {
            nums.remove(v - i);
        }
        for _ in 0..len {
            nums.push(0);
        }
    }

    #[allow(unused)]
    pub fn impl2(nums: &mut Vec<i32>) {
        for i in 1..nums.len() {
            for j in 0..nums.len() - i {
                if nums[j] == 0 {
                    nums[j] = nums[j + 1];
                    nums[j + 1] = 0;
                }
            }
        }
    }

    #[allow(unused)]
    pub fn impl3(nums: &mut Vec<i32>) {
        let len = nums.len();
        nums.retain(|&x| x != 0);
        nums.resize(len, 0);
    }

    #[allow(unused)]
    pub fn impl4(nums: &mut Vec<i32>) {
        let len = nums.len();
        let mut left = 0;
        let mut right = 0;
        while right < len {
            if nums[right] != 0 {
                nums.swap(right, left);
                left += 1;
            }
            right += 1;
        }
    }

    pub fn move_zeroes(nums: &mut Vec<i32>) {
        Solution::impl1(nums)
    }
}

fn main() {
    let mut nums = vec![0, 1, 0, 3, 12];
    Solution::move_zeroes(&mut nums);
    println!("{:?}", nums);
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_impl1() {
        let mut nums = vec![0, 1, 0, 3, 12];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 3, 12, 0, 0]);

        let mut nums = vec![0];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![0]);
    }

    #[test]
    fn test_impl2() {
        let mut nums = vec![0, 1, 0, 3, 12];
        Solution::impl2(&mut nums);
        assert_eq!(nums, vec![1, 3, 12, 0, 0]);

        let mut nums = vec![0];
        Solution::impl2(&mut nums);
        assert_eq!(nums, vec![0]);
    }

    #[test]
    fn test_impl3() {
        let mut nums = vec![0, 1, 0, 3, 12];
        Solution::impl3(&mut nums);
        assert_eq!(nums, vec![1, 3, 12, 0, 0]);

        let mut nums = vec![0];
        Solution::impl3(&mut nums);
        assert_eq!(nums, vec![0]);
    }

    #[test]
    fn test_impl4() {
        let mut nums = vec![0, 1, 0, 3, 12];
        Solution::impl4(&mut nums);
        assert_eq!(nums, vec![1, 3, 12, 0, 0]);

        let mut nums = vec![0];
        Solution::impl4(&mut nums);
        assert_eq!(nums, vec![0]);
    }
}
