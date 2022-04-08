struct Solution;

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        use std::collections::HashSet;

        let (short, long) = if nums1.len() > nums2.len() {
            (nums2, nums1)
        } else {
            (nums1, nums2)
        };
        let mut ans: HashSet<(i32, usize)> = HashSet::new();
        for i in short {
            for (j, v) in long.iter().enumerate() {
                if i == *v && ans.insert((i, j)) {
                    break;
                }
            }
        }
        ans.into_iter().map(|(i, _)| i).collect::<Vec<i32>>()
    }
}

fn main() {
    println!("{:?}", Solution::intersect(vec![1, 2, 2, 1], vec![2, 2]));
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_impl1() {
        let tuple = vec![
            (vec![1, 2, 2, 1], vec![2, 2], vec![2, 2]),
            (vec![4, 9, 5], vec![9, 4, 9, 8, 4], vec![4, 9]),
            (vec![1, 1, 1], vec![1], vec![1]),
            (vec![1], vec![1], vec![1]),
            (vec![3, 1, 2], vec![1, 1], vec![1]),
        ];
        for (nums1, nums2, result) in tuple {
            let mut r = Solution::intersect(nums1, nums2);
            r.sort_unstable();
            assert_eq!(r, result);
        }
    }
}
