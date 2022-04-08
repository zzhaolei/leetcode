struct Solution;

impl Solution {
    pub fn next_permutation(vec: &mut [i32]) {
        let reverse = |a: &mut [i32]| {
            let mut i = 0;
            let n = a.len();
            while i < n / 2 {
                a.swap(i, n - i - 1);
                i += 1;
            }
        };
        let n = vec.len();
        let mut i = (n - 2) as i32;
        while i >= 0 && vec[i as usize] >= vec[(i + 1) as usize] {
            i -= 1;
        }
        if i >= 0 {
            let mut j = (n - 1) as i32;
            while j >= 0 && vec[i as usize] >= vec[j as usize] {
                j -= 1;
            }
            vec.swap(i as usize, j as usize);
            i += 1;
        } else {
            i = 0;
        }
        reverse(&mut vec[i as usize..])
    }
}

fn main() {
    let mut v = vec![1, 2, 3];
    Solution::next_permutation(&mut v);
    println!("{v:?}");
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        let mut vec = vec![1, 2, 3];
        Solution::next_permutation(&mut vec);
        assert_eq!(vec, [1, 3, 2]);
        let mut vec = vec![3, 2, 1];
        Solution::next_permutation(&mut vec);
        assert_eq!(vec, [1, 2, 3]);
        let mut vec = vec![1, 1, 5];
        Solution::next_permutation(&mut vec);
        assert_eq!(vec, [1, 5, 1]);
    }
}
