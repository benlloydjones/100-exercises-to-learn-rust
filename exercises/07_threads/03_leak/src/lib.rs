// TODO: Given a vector of integers, leak its heap allocation.
//  Then split the resulting static slice into two halves and
//  sum each half in a separate thread.
//  Hint: check out `Vec::leak`.

use std::thread::{self, JoinHandle};

pub fn sum(v: Vec<i32>) -> i32 {
    let a: &'static mut [i32] = v[..v.len() / 2].to_vec().leak();
    let b: &'static mut [i32] = v[v.len() / 2..].to_vec().leak();

    let handle_a: JoinHandle<i32> = thread::spawn(|| a.iter().sum());
    let handle_b: JoinHandle<i32> = thread::spawn(|| b.iter().sum());

    handle_a.join().unwrap() + handle_b.join().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
