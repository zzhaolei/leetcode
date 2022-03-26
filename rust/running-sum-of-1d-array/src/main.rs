struct Solution;

impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut ans: Vec<i32> = Vec::new();

        
        for i in 0..nums.len() {
            let mut temp = 0;
            for j in nums.iter().take(i + 1) {
                temp += j;
            }
            ans.push(temp)
        }
        ans
    }
}

fn main() {
    println!("{:?}", Solution::running_sum(vec![1, 2, 3, 4]));
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::running_sum(vec![1, 2, 3, 4]), [1, 3, 6, 10]);
    }
}
