struct CQueue {
    in_stack: Vec<i32>,
    out_stack: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CQueue {
    fn new() -> Self {
        // 1 <= values <= 10000
        Self {
            in_stack: Vec::with_capacity(10000),
            out_stack: Vec::with_capacity(10000),
        }
    }

    fn append_tail(&mut self, value: i32) {
        self.in_stack.push(value);
    }

    fn delete_head(&mut self) -> i32 {
        if self.out_stack.len() == 0 {
            if self.in_stack.len() == 0 {
                return -1;
            }
            self.out_stack.append(&mut self.in_stack);
        }
        let value = self.out_stack[0];
        self.out_stack = self.out_stack[1..].to_vec();
        value
    }
}

/**
 * Your CQueue object will be instantiated and called as such:
 * let obj = CQueue::new();
 * obj.append_tail(value);
 * let ret_2: i32 = obj.delete_head();
 */

fn main() {
    let mut q = CQueue::new();
    q.append_tail(3);
    let v = q.delete_head();
    println!("{v}");
    let v = q.delete_head();
    println!("{v}");
    let v = q.delete_head();
    println!("{v}");
}
