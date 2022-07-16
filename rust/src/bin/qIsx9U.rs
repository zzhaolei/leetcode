#![allow(non_snake_case)]

struct MovingAverage {
    vec: Vec<i32>,
    len: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MovingAverage {
    /** Initialize your data structure here. */
    fn new(size: i32) -> Self {
        MovingAverage {
            len: size as usize,
            vec: Vec::new(),
        }
    }

    fn next(&mut self, val: i32) -> f64 {
        if self.vec.len() >= self.len {
            self.vec.pop();
        }
        self.vec.insert(0, val);
        self.vec.iter().sum::<i32>() as f64 / self.vec.len() as f64
    }
}

/**
 * Your MovingAverage object will be instantiated and called as such:
 * let obj = MovingAverage::new(size);
 * let ret_1: f64 = obj.next(val);
 */

fn main() {
    let mut obj = MovingAverage::new(3);
    println!("{}", obj.next(1));
    println!("{}", obj.next(10));
    println!("{}", obj.next(3));
    println!("{}", obj.next(5));

    let mut obj = MovingAverage::new(1);
    println!("{}", obj.next(4));
    println!("{}", obj.next(0));
}

#[cfg(test)]
mod tests {
    use crate::MovingAverage;

    #[test]
    fn test() {
        let mut obj = MovingAverage::new(3);
        assert_eq!(obj.next(1), 1.0);
        assert_eq!(obj.next(10), 5.5);
        assert_eq!(obj.next(3), 4.666666666666667);
        assert_eq!(obj.next(5), 6.0);

        let mut obj = MovingAverage::new(1);
        assert_eq!(obj.next(4), 4.0);
        assert_eq!(obj.next(0), 0.0);
    }
}
