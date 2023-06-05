struct Solution;

impl Solution {
    pub fn apply_operations(mut nums: Vec<i32>) -> Vec<i32> {
        for i in 0..nums.len() - 1 {
            if nums[i] == nums[i + 1] {
                nums[i] *= 2;
                nums[i + 1] = 0;
            }
        }

        let (mut i, mut j) = (0, 1);
        while j < nums.len() {
            if nums[i] == 0 {
                if nums[j] == 0 {
                    j += 1;
                    continue;
                }
                nums.swap(i, j);
            }
            i += 1;
            j += 1;
        }
        nums
    }
}

fn main() {
    println!("{:?}", Solution::apply_operations(vec![0, 1]));
    println!(
        "{:?}",
        Solution::apply_operations(vec![300, 126, 0, 0, 523, 523])
    );
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use crate::Solution;

        assert_eq!(Solution::apply_operations(vec![0, 1]), [1, 0]);
        assert_eq!(
            Solution::apply_operations(vec![300, 126, 0, 0, 523, 523]),
            [300, 126, 1046, 0, 0, 0]
        );
    }
}
