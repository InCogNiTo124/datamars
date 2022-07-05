use crate::operator;
use std::collections::BinaryHeap;
use ordered_float::NotNan;
use std::cmp::{Ordering, Reverse};

// TODO(msmetko) implement Reverse

// implements the algorithm as defined in
// https://web.archive.org/web/20220627211752/http://www.dsalgo.com/2013/02/RunningMedian.php.html
pub struct Median {
    low_heap: BinaryHeap<NotNan<f64>>,
    high_heap: BinaryHeap<Reverse<NotNan<f64>>>
}

impl operator::Operator for Median {
    fn apply(&mut self, x: f64) {
        // addint the number to the proper heap

        let new_val = NotNan::new(x).expect("");
        if x >= (self.high_heap.peek().expect("").0.into_inner()) {
            self.high_heap.push(Reverse(new_val));
        } else {
            self.low_heap.push(new_val);
        }

        // balancing the heaps
        let size_dif = self.high_heap.len() as i64 - self.low_heap.len() as i64;
        if size_dif == 2 {
            let new_val = self.high_heap.pop().expect("").0.into_inner();
            self.low_heap.push(NotNan::new(new_val).expect(""));
        } else if size_dif == -2 {
            let new_val = self.low_heap.pop().expect("").into();
            self.high_heap.push(Reverse(NotNan::new(new_val).expect("")));
        }
    }

    fn result(&self) -> f64 {
        // returning the median
        match self.high_heap.len().cmp(&self.low_heap.len()) {
            Ordering::Greater => (self.high_heap.peek().expect("").0.into_inner() + self.low_heap.peek().expect("").into_inner()) / 2.0,
            Ordering::Less => self.high_heap.peek().expect("").0.into_inner(),
            Ordering::Equal => self.low_heap.peek().expect("").into_inner()
        }
        // if self.high_heap.len() == self.low_heap.len() {
        //     return
        // } else if self.high_heap.len() > self.low_heap.len() {
        //     return
        // } else {
        //     return
        // }
    }
}

impl Median {
    pub fn new() -> Median {

        let mut low_heap = BinaryHeap::new();
        low_heap.push(NotNan::new(f64::NEG_INFINITY).expect(""));
        let mut high_heap = BinaryHeap::new();
        high_heap.push(Reverse(NotNan::new(f64::INFINITY).expect("")));
        Median {
            low_heap,
            high_heap
        }
    }
}
