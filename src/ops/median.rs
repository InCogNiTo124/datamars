use crate::ops::Operator;
use ordered_float::NotNan;
use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;

// implements the algorithm as defined in
// https://web.archive.org/web/20220627211752/http://www.dsalgo.com/2013/02/RunningMedian.php.html
pub struct Median {
    low_heap: BinaryHeap<NotNan<f64>>,
    high_heap: BinaryHeap<Reverse<NotNan<f64>>>,
}

impl Operator for Median {
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
            self.high_heap
                .push(Reverse(NotNan::new(new_val).expect("")));
        }
        assert!(self.high_heap.len().abs_diff(self.low_heap.len()) <= 1);
    }

    fn result(&self) -> f64 {
        // returning the median
        let high = self.high_heap.peek().expect("").0.into_inner();
        let low = self.low_heap.peek().expect("").into_inner();
        if self.high_heap.len() == 1 && self.low_heap.len() == 1 {
            return f64::NAN;
        } else if self.high_heap.len() == 2 && self.low_heap.len() == 1 {
            return high;
        } else if self.low_heap.len() == 2 && self.high_heap.len() == 1 {
            return low;
        } else {
            match self.high_heap.len().cmp(&self.low_heap.len()) {
                Ordering::Greater => high,
                Ordering::Less => low,
                Ordering::Equal => (high + low) / 2.0,
            }
        }
    }
}

impl Median {
    pub fn new() -> Self {
        let mut low_heap = BinaryHeap::new();
        low_heap.push(NotNan::new(f64::NEG_INFINITY).expect(""));
        let mut high_heap = BinaryHeap::new();
        high_heap.push(Reverse(NotNan::new(f64::INFINITY).expect("")));
        Self {
            low_heap,
            high_heap,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Median, Operator};

    #[test]
    fn test1() {
        let mut obj = Median::new();
        let test_cases = vec![
            (1, 1.0),
            (2, 1.5),
            (3, 2.0),
            (4, 2.5),
            (5, 3.0),
            (6, 3.5),
            (7, 4.0),
            (8, 4.5),
            (9, 5.0),
            (100, 5.5),
        ];
        for (x, y) in test_cases {
            obj.apply(x as f64);
            let error = (obj.result() - y).abs();
            assert!(error < 1e-15);
        }
    }
}
