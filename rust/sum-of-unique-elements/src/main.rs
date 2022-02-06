struct Solution;

impl Solution {
    pub fn impl1(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let mut map: HashMap<i32, i32> = HashMap::new();
        for i in nums.into_iter() {
            if let Some(val) = map.get_mut(&i) {
                *val += 1;
            } else {
                map.insert(i, 0);
            }
        }
        let mut ans = 0;
        for (k, v) in map {
            if v == 0 {
                ans += k;
            }
        }
        ans
    }

    #[allow(unused)]
    pub fn impl2(nums: Vec<i32>) -> i32 {
        let mut array = [0; 101];
        for i in 0..nums.len() {
            array[nums[i] as usize] += 1;
        }
        array
            .into_iter()
            .enumerate()
            .map(|(i, x)| if x == 1 { i as i32 } else { 0 })
            .sum()
    }

    #[allow(unused)]
    pub fn impl3(nums: Vec<i32>) -> i32 {
        let mut array = [0; 101];
        let mut ans = 0;
        for i in nums.into_iter() {
            let x = array[i as usize];
            if x == 0 {
                ans += i;
            } else if x == 1 {
                ans -= i;
            }
            array[i as usize] += 1;
        }
        ans
    }

    pub fn sum_of_unique(nums: Vec<i32>) -> i32 {
        Solution::impl1(nums)
    }
}

fn main() {
    println!("{}", Solution::sum_of_unique(vec![1, 2, 3, 2]));
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_impl1() {
        assert_eq!(Solution::impl1(vec![1, 2, 3, 2]), 4);
        assert_eq!(Solution::impl1(vec![1, 1, 1, 1, 1]), 0);
        assert_eq!(Solution::impl1(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn test_impl2() {
        assert_eq!(Solution::impl2(vec![1, 2, 3, 2]), 4);
        assert_eq!(Solution::impl2(vec![1, 1, 1, 1, 1]), 0);
        assert_eq!(Solution::impl2(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn test_impl3() {
        assert_eq!(Solution::impl3(vec![1, 2, 3, 2]), 4);
        assert_eq!(Solution::impl3(vec![1, 1, 1, 1, 1]), 0);
        assert_eq!(Solution::impl3(vec![1, 2, 3, 4, 5]), 15);
    }
}
