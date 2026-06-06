pub struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[allow(unused)]
struct KthLargest {
    k: usize,
    heap: BinaryHeap<Reverse<i32>>,
}

impl KthLargest {
    #[allow(unused)]
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut kth_largest = KthLargest {
            k: k as usize,
            heap: BinaryHeap::new(),
        };

        for num in nums {
            kth_largest.add(num);
        }
        kth_largest
    }

    #[allow(unused)]
    fn add(&mut self, val: i32) -> i32 {
        self.heap.push(Reverse(val));

        if self.heap.len() > self.k {
            self.heap.pop();
        }

        let Reverse(ans) = self.heap.peek().unwrap();
        *ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kthlargest() {
        let mut obj = KthLargest::new(3, vec![4, 5, 8, 2]);
        assert_eq!(obj.add(3), 4);
        assert_eq!(obj.add(5), 5);
        assert_eq!(obj.add(10), 5);
        assert_eq!(obj.add(9), 8);
        assert_eq!(obj.add(4), 8);
    }
}
