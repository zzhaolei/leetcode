struct MyCalendar {
    date: Vec<(i32, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendar {
    fn new() -> Self {
        MyCalendar { date: vec![] }
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        for i in self.date.iter() {
            if i.0 < end && start < i.1 {
                return false;
            }
        }
        self.date.push((start, end));
        true
    }
}

/**
 * Your MyCalendar object will be instantiated and called as such:
 * let obj = MyCalendar::new();
 * let ret_1: bool = obj.book(start, end);
 */

fn main() {
    let mut my = MyCalendar::new();
    println!("{}", my.book(10, 20));
    println!("{}", my.book(15, 25));
    println!("{}", my.book(20, 30));
}

#[cfg(test)]
mod tests {
    use crate::MyCalendar;

    #[test]
    fn test() {
        let mut my = MyCalendar::new();
        assert!(my.book(10, 20));
        assert!(!my.book(15, 25));
        assert!(my.book(20, 30));
    }
}
