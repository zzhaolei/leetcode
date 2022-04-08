struct Solution;

impl Solution {
    pub fn valid_mountain_array(a: Vec<i32>) -> bool {
        let length = a.len();
        if length < 3 || a[0] > a[1] || a[length - 1] > a[length - 2] {
            return false;
        }
        let mut find = false;
        for (i, v) in a[..length - 1].iter().enumerate() {
            let next_value = &a[i + 1];
            if v == next_value {
                return false;
            }
            if !find && v > next_value {
                find = true
            }
            if find && v < next_value {
                return false;
            }
        }
        true
    }
}

fn main() {
    println!("{}", Solution::valid_mountain_array(vec![2, 1]));
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        assert!(!Solution::valid_mountain_array(vec![2, 1]));
        assert!(!Solution::valid_mountain_array(vec![3, 5, 5]));
        assert!(Solution::valid_mountain_array(vec![0, 3, 2, 1]));
    }
}
