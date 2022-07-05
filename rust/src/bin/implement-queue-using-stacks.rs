#![allow(unused)]

struct MyQueue {
    inner: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {
    fn new() -> Self {
        MyQueue { inner: Vec::new() }
    }

    fn push(&mut self, x: i32) {
        self.inner.push(x);
    }

    fn pop(&mut self) -> i32 {
        self.inner.remove(0)
    }

    fn peek(&self) -> i32 {
        if let Some(x) = self.inner.first() {
            *x
        } else {
            0
        }
    }

    fn empty(&self) -> bool {
        self.inner.is_empty()
    }
}

/**
 * Your MyQueue object will be instantiated and called as such:
 * let obj = MyQueue::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.peek();
 * let ret_4: bool = obj.empty();
 */

fn main() {
    let mut obj = MyQueue::new();
    obj.push(1);
    obj.push(2);
}

#[cfg(test)]
mod tests {
    use crate::MyQueue;

    #[test]
    fn test() {
        let mut obj = MyQueue::new();
        obj.push(1);
        obj.push(2);
        assert_eq!(obj.peek(), 1);
        assert_eq!(obj.pop(), 1);
        assert_eq!(obj.empty(), false);
    }
}
