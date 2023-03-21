struct Solution;

impl Solution {
    pub fn convert_temperature(celsius: f64) -> Vec<f64> {
        Vec::from([celsius + 273.15, celsius * 1.80 + 32.00])
    }
}

fn main() {
    println!("{:?}", Solution::convert_temperature(36.50));
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::convert_temperature(36.50), [309.65, 97.7]);
    }
}
