struct OrderedStream {
    buf: Vec<(i32, String)>,
    ptr: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl OrderedStream {
    fn new(n: i32) -> Self {
        Self {
            buf: vec![(0, "".to_owned()); n as usize],
            ptr: 1,
        }
    }

    fn insert(&mut self, id_key: i32, value: String) -> Vec<String> {
        self.buf[id_key as usize - 1] = (id_key, value.clone());
        let mut ans = vec![];
        if id_key == self.ptr {
            ans.push(value);
            for i in self.buf[id_key as usize..].iter() {
                if i.0 > self.ptr {
                    ans.push(i.1.clone());
                    self.ptr = i.0;
                } else {
                    self.ptr += 1;
                    break;
                }
            }
        }
        ans
    }
}

/**
 * Your OrderedStream object will be instantiated and called as such:
 * let obj = OrderedStream::new(n);
 * let ret_1: Vec<String> = obj.insert(idKey, value);
 */

fn main() {
    let mut obj = OrderedStream::new(5);
    println!("{:?}", obj.insert(3, "ccccc".to_owned()));
    println!("{:?}", obj.insert(1, "aaaaa".to_owned()));
    println!("{:?}", obj.insert(2, "bbbbb".to_owned()));
    println!("{:?}", obj.insert(5, "eeeee".to_owned()));
    println!("{:?}", obj.insert(4, "ddddd".to_owned()));
}

#[cfg(test)]
mod tests {
    use crate::OrderedStream;

    #[test]
    fn test() {
        let mut obj = OrderedStream::new(5);
        assert!(obj.insert(3, "ccccc".to_owned()).is_empty());
        assert_eq!(obj.insert(1, "aaaaa".to_owned()), ["aaaaa".to_owned()]);
        assert_eq!(
            obj.insert(2, "bbbbb".to_owned()),
            ["bbbbb".to_owned(), "ccccc".to_owned()]
        );
        assert!(obj.insert(5, "eeeee".to_owned()).is_empty());
        assert_eq!(
            obj.insert(4, "ddddd".to_owned()),
            ["ddddd".to_owned(), "eeeee".to_owned()]
        );
    }
}
