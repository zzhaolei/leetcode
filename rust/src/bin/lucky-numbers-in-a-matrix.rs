struct Solution;

impl Solution {
    pub fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        for v in matrix.iter() {
            let mut ans = *v.first().unwrap();
            let mut index = 0;
            for (i, v) in v.iter().enumerate() {
                if *v < ans {
                    ans = *v;
                    index = i;
                }
            }
            let mut find = true;
            for v in matrix.iter() {
                if ans < v[index] {
                    find = false;
                    break;
                }
            }
            if find {
                return vec![ans];
            }
        }
        vec![]
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::lucky_numbers(vec![vec![3, 7, 8], vec![9, 11, 13], vec![15, 16, 17]])
    );
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_impl1() {
        assert_eq!(
            Solution::lucky_numbers(vec![vec![3, 7, 8], vec![9, 11, 13], vec![15, 16, 17]]),
            vec![15]
        );
        assert_eq!(
            Solution::lucky_numbers(vec![
                vec![1, 10, 4, 2],
                vec![9, 3, 8, 7],
                vec![15, 16, 17, 12]
            ]),
            vec![12]
        );
        assert_eq!(
            Solution::lucky_numbers(vec![vec![7, 8], vec![1, 2]]),
            vec![7]
        );
    }
}
