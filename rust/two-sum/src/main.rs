struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        let mut map: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
        for (index, value) in nums.iter().enumerate() {
            let t = target - value;
            if let Some(r) = map.get(&value) {
                result.push(*r);
                result.push(index as i32);
            }
            map.insert(t, index as i32);
        }
        result
    }
}

fn main() {
    println!("{:?}", Solution::two_sum(vec![2, 7, 11, 15], 9));
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_impl1() {
        let t = vec![
            (vec![2, 7, 11, 15], 9, vec![0, 1]),
            (vec![3, 2, 4], 6, vec![1, 2]),
            (vec![3, 3], 6, vec![0, 1]),
        ];
        for (nums, target, result) in t {
            let mut r = Solution::two_sum(nums, target);
            r.sort_unstable();
            assert_eq!(r, result);
        }
    }
}
