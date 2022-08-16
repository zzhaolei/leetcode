struct Solution;

impl Solution {
    fn impl1(m: i32, n: i32, indices: Vec<Vec<i32>>) -> i32 {
        let mut matrix = vec![vec![0; n as usize]; m as usize];
        for i in indices.iter() {
            let a = i[0] as usize;
            let b = i[1] as usize;

            for (j, v) in matrix.iter_mut().enumerate() {
                v[b] += 1;
                if j == a {
                    for k in v.iter_mut() {
                        *k += 1;
                    }
                }
            }
        }
        let mut ans = 0;
        for v in matrix.iter() {
            for k in v.iter() {
                if k % 2 != 0 {
                    ans += 1;
                }
            }
        }
        ans
    }
    fn impl2(m: i32, n: i32, indices: Vec<Vec<i32>>) -> i32 {
        let mut rows = vec![0; m as usize];
        let mut cols = vec![0; n as usize];
        for i in indices {
            rows[i[0] as usize] ^= 1;
            cols[i[1] as usize] ^= 1;
        }
        let r: i32 = rows.iter().sum();
        let c: i32 = cols.iter().sum();
        r * (n - c) + c * (m - r)
    }

    pub fn odd_cells(m: i32, n: i32, indices: Vec<Vec<i32>>) -> i32 {
        // Solution::impl1(m, n, indices)
        Solution::impl2(m, n, indices)
    }
}

fn main() {
    println!("{}", Solution::impl1(2, 3, vec![vec![0, 1], vec![1, 1]]));
    println!(
        "{}",
        Solution::odd_cells(2, 2, vec![vec![1, 1], vec![0, 0]])
    );
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::odd_cells(2, 3, vec![vec![0, 1], vec![1, 1]]), 6);
        assert_eq!(Solution::odd_cells(2, 2, vec![vec![1, 1], vec![0, 0]]), 0);
    }
}
