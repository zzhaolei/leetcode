struct Solution;

impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut ans = vec![];
        let (left, right) = (&nums[..n], &nums[n..]);
        let mut j = 0;
        for (i, &v) in left.iter().enumerate() {
            j = i;
            ans.push(v);
            ans.push(right[j]);
        }
        for &i in &right[j + 1..] {
            ans.push(i);
        }
        ans
    }
}

fn main() {
    println!("{:?}", Solution::shuffle(vec![1, 2, 3, 4, 4, 3, 2, 1], 4));
    println!("{:?}", Solution::shuffle(vec![1, 1, 2, 2], 2));
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::shuffle(vec![1, 2, 3, 4, 4, 3, 2, 1], 4),
            [1, 4, 2, 3, 3, 2, 4, 1]
        );
        assert_eq!(Solution::shuffle(vec![1, 1, 2, 2], 2), [1, 2, 1, 2]);
    }
}
