#![allow(unused)]
/// https://leetcode-cn.com/problems/design-parking-system/
use std::sync::atomic::AtomicI32;
struct ParkingSystem {
    big: AtomicI32,
    medium: AtomicI32,
    small: AtomicI32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ParkingSystem {
    fn new(big: i32, medium: i32, small: i32) -> Self {
        Self {
            big: AtomicI32::new(big),
            medium: AtomicI32::new(medium),
            small: AtomicI32::new(small),
        }
    }

    fn add_car(&mut self, car_type: i32) -> bool {
        let get_car_parking = |x: &mut i32| {
            if *x - 1 < 0 {
                false
            } else {
                *x -= 1;
                true
            }
        };
        match car_type {
            1 => get_car_parking(&mut (*self.big.get_mut())),
            2 => get_car_parking(&mut (*self.medium.get_mut())),
            3 => get_car_parking(&mut (*self.small.get_mut())),
            _ => false,
        }
    }
}

/**
 * Your ParkingSystem object will be instantiated and called as such:
 * let obj = ParkingSystem::new(big, medium, small);
 * let ret_1: bool = obj.add_car(carType);
 */

fn main() {}
