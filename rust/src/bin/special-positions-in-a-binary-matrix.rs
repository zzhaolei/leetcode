struct Solution;

impl Solution {
    pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        for i in 0..mat.len() {
            if mat[i].iter().filter(|x| **x == 1).count() != 1 {
                continue;
            }
            for j in 0..mat[i].len() {
                if mat[i][j] != 1 {
                    continue;
                }
                let mut vec = vec![];
                for v in &mat {
                    vec.push(v[j]);
                }
                if vec.iter().filter(|x| **x == 1).count() != 1 {
                    continue;
                }
                ans += 1;
            }
        }
        ans
    }
}

fn main() {
    println!(
        "{}",
        Solution::num_special(vec![vec![1, 0, 0], vec![0, 0, 1], vec![1, 0, 0]])
    );
    println!(
        "{}",
        Solution::num_special(vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]])
    );
    println!(
        "{}",
        Solution::num_special(vec![
            vec![0, 0, 0, 1],
            vec![1, 0, 0, 0],
            vec![0, 1, 1, 0],
            vec![0, 0, 0, 0]
        ])
    );
    println!(
        "{}",
        Solution::num_special(vec![
            vec![0, 0, 0, 0, 0],
            vec![1, 0, 0, 0, 0],
            vec![0, 1, 0, 0, 0],
            vec![0, 0, 1, 0, 0],
            vec![0, 0, 0, 1, 1]
        ])
    );
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        assert_eq!(
            1,
            Solution::num_special(vec![vec![1, 0, 0], vec![0, 0, 1], vec![1, 0, 0]])
        );
        assert_eq!(
            3,
            Solution::num_special(vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]])
        );
        assert_eq!(
            2,
            Solution::num_special(vec![
                vec![0, 0, 0, 1],
                vec![1, 0, 0, 0],
                vec![0, 1, 1, 0],
                vec![0, 0, 0, 0]
            ])
        );
        assert_eq!(
            3,
            Solution::num_special(vec![
                vec![0, 0, 0, 0, 0],
                vec![1, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 0],
                vec![0, 0, 1, 0, 0],
                vec![0, 0, 0, 1, 1]
            ])
        );
    }
}
