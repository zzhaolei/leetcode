struct Solution;

impl Solution {
    pub fn array_nesting(nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;

        let mut ans = 0;
        let mut store = HashSet::new();
        let mut set = HashSet::new();

        for i in 0..nums.len() {
            let mut v = nums[i];
            if let Some(_) = store.get(&v) {
                continue;
            }
            loop {
                if !set.insert(v) {
                    break;
                }
                v = nums[v as usize];
                store.insert(v);
            }
            ans = ans.max(set.len());
            set.clear();
        }
        ans as i32
    }
}

fn main() {
    println!("{}", Solution::array_nesting(vec![5, 4, 0, 3, 1, 6, 2]));
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::array_nesting(vec![5, 4, 0, 3, 1, 6, 2]), 4);
    }
}
