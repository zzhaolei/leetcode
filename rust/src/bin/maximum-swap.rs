struct Solution;

impl Solution {
    pub fn maximum_swap(num: i32) -> i32 {
        let mut v = num.to_string().chars().collect::<Vec<char>>();
        for i in 0..v.len() - 1 {
            let mut max = (i, v[i]);
            for (j, &item) in v.iter().enumerate().skip(i + 1) {
                if max.1 <= item {
                    max.1 = max.1.max(item);
                    max.0 = j;
                }
            }
            if max.1 > v[i] {
                v.swap(i, max.0);
                break;
            }
        }
        v.into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("")
            .parse::<i32>()
            .unwrap_or(0)
    }
}

fn main() {
    println!("{}", Solution::maximum_swap(2736));
    println!("{}", Solution::maximum_swap(9973));
    println!("{}", Solution::maximum_swap(1993));
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        assert_eq!(7236, Solution::maximum_swap(2736));
        assert_eq!(9973, Solution::maximum_swap(9973));
        assert_eq!(9913, Solution::maximum_swap(1993));
    }
}
