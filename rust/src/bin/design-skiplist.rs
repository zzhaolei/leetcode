use std::collections::HashMap;

struct Skiplist {
    skip_list: HashMap<i32, u16>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Skiplist {
    fn new() -> Self {
        Self {
            skip_list: HashMap::new(),
        }
    }

    fn search(&self, target: i32) -> bool {
        self.skip_list.get(&target).is_some()
    }

    fn add(&mut self, num: i32) {
        match self.skip_list.get_mut(&num) {
            Some(v) => *v += 1,
            None => {
                self.skip_list.insert(num, 1);
            }
        }
    }

    fn erase(&mut self, num: i32) -> bool {
        if self.skip_list.get(&num).is_none() {
            return false;
        }
        let v = unsafe { self.skip_list.get_mut(&num).unwrap_unchecked() };
        *v -= 1;
        if *v == 0 {
            self.skip_list.remove(&num);
        }
        true
    }
}

/**
 * Your Skiplist object will be instantiated and called as such:
 * let obj = Skiplist::new();
 * let ret_1: bool = obj.search(target);
 * obj.add(num);
 * let ret_3: bool = obj.erase(num);
 */

fn main() {
    let mut obj = Skiplist::new();
    obj.add(1);
    obj.add(2);
    obj.add(3);
    println!("{:?} = false", obj.search(0));
    obj.add(4);
    println!("{:?} = true", obj.search(1));
    println!("{:?} = false", obj.erase(0));
    println!("{:?} = true", obj.erase(1));
    println!("{:?} = false", obj.search(1));
}

#[cfg(test)]
mod tests {
    use crate::Skiplist;

    #[test]
    fn test() {
        let mut obj = Skiplist::new();
        obj.add(1);
        obj.add(2);
        obj.add(3);
        assert!(!obj.search(0));
        obj.add(4);
        assert!(obj.search(1));
        assert!(!obj.erase(0));
        assert!(obj.erase(1));
        assert!(!obj.search(1));
    }
}
