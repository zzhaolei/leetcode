struct Solution;

impl Solution {
    fn impl1(nums: Vec<i32>) -> bool {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        for i in nums {
            if map.get(&i).is_some() {
                return true;
            } else {
                map.insert(i, ());
            }
        }
        false
    }

    #[allow(unused)]
    fn impl2(nums: Vec<i32>) -> bool {
        use std::collections::HashSet;
        use std::iter::FromIterator;

        let set: HashSet<&i32> = HashSet::from_iter(nums.iter());
        set.len() != nums.len()
    }

    #[allow(unused)]
    fn impl3(nums: Vec<i32>) -> bool {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        nums.iter().any(|&x| !set.insert(x))
    }

    #[allow(unused)]
    fn impl4(mut nums: Vec<i32>) -> bool {
        nums.sort_unstable();
        for i in 0..nums.len() - 1 {
            if nums[i] == nums[i + 1] {
                return true;
            }
        }
        false
    }

    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        Solution::impl1(nums)
    }
}

fn main() {
    println!("{}", Solution::contains_duplicate(vec![1, 2, 3, 1]));
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_impl1() {
        assert_eq!(Solution::impl1(vec![1, 2, 3, 1]), true);
        assert_eq!(Solution::impl1(vec![1, 2, 3, 4]), false);
        assert_eq!(Solution::impl1(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]), true);
    }

    #[test]
    fn test_impl2() {
        assert_eq!(Solution::impl2(vec![1, 2, 3, 1]), true);
        assert_eq!(Solution::impl2(vec![1, 2, 3, 4]), false);
        assert_eq!(Solution::impl2(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]), true);
    }

    #[test]
    fn test_impl3() {
        assert_eq!(Solution::impl3(vec![1, 2, 3, 1]), true);
        assert_eq!(Solution::impl3(vec![1, 2, 3, 4]), false);
        assert_eq!(Solution::impl3(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]), true);
    }

    #[test]
    fn test_impl4() {
        assert_eq!(Solution::impl4(vec![1, 2, 3, 1]), true);
        assert_eq!(Solution::impl4(vec![1, 2, 3, 4]), false);
        assert_eq!(Solution::impl4(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]), true);
    }
}
