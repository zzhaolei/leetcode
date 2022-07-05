struct Solution;

// The API isBadVersion is defined for you.
// isBadVersion(versions:i32)-> bool;
// to call it use self.isBadVersion(versions)

impl Solution {
    /// leetcode自己提供这个函数，在这里定义函数是为了防止报错
    #[allow(non_snake_case)]
    fn isBadVersion(&self, _n: i32) -> bool {
        true
    }

    pub fn first_bad_version(&self, n: i32) -> i32 {
        let (mut left, mut right) = (0, n);
        let mut mid;
        let mut ans = 0;
        while left <= right {
            mid = left + ((right - left) / 2);
            if self.isBadVersion(mid) {
                right = mid - 1;
                ans = mid;
            } else {
                left = mid + 1;
            }
        }
        ans
    }
}

fn main() {
    let s = Solution;
    println!("{}", s.first_bad_version(5));
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        let s = Solution;
        assert_eq!(s.first_bad_version(5), 0);
    }
}
