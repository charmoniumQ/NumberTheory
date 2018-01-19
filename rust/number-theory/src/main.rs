use std::ops::Index;
use std::ops::Range;
use std::ops::RangeFrom;
use std::ops::RangeTo;
use std::vec::Vec;

struct SortedVec<T: PartialOrd>(Vec<T>);

impl<T: PartialOrd> Index<usize> for SortedVec<T> {
    type Output = T;
    fn index(&self, index: usize) -> &T { &self.0.index(index) }
}

impl<T: PartialOrd> Index<Range<usize>> for SortedVec<T> {
    type Output = [T];
    fn index(&self, index: Range<usize>) -> &[T] { &self.0.index(index) }
}

impl<T: PartialOrd> Index<RangeTo<usize>> for SortedVec<T> {
    type Output = [T];
    fn index(&self, index: RangeTo<usize>) -> &[T] { &self.0.index(index) }
}

impl<T: PartialOrd> Index<RangeFrom<usize>> for SortedVec<T> {
    type Output = [T];
    fn index(&self, index: RangeFrom<usize>) -> &[T] { &self.0.index(index) }
}

impl<T: PartialOrd> SortedVec<T> {
    fn valid(&self) -> bool {
        if ! self.is_empty() {
            let mut last: &T = &self[0_usize];
            for elem in &self[1_usize..] {
                if elem >= last {
                    return false;
                }
                last = elem;
            }
        }
        true
    }

    fn contains(&self, elem: &T) -> bool {
        if ! self.is_empty() {
            let mid = self.len() / 2;
            if elem < &self[mid] {
                return self[..mid].contains(elem);
            } else if &self[mid] < elem {
                return self[mid..].contains(elem);
            } else {
                return true;
            }
        }
        false
    }

    fn len(&self) -> usize { self.0.len() }
    fn is_empty(&self) -> bool { self.0.is_empty() }
}

// struct SortedVec<T: PartialOrd> {
//     data: Vec<T>,
// }

// impl<T: PartialOrd> Index<usize> for SortedVec<T> {
//     type Output = T;
//     fn index(&self, index: usize) -> &T { &self.data.index(index) }
// }
// impl<T: PartialOrd> Index<RangeTo<usize>> for SortedVec<T> {
//     type Output = [T];
//     fn index(&self, index: RangeTo<usize>) -> &[T] { &self.data.index(index) }
// }
// impl<T: PartialOrd> Index<RangeFrom<usize>> for SortedVec<T> {
//     type Output = [T];
//     fn index(&self, index: RangeFrom<usize>) -> &[T] { &self.data.index(index) }
// }

// impl<T: PartialOrd> SortedVec<T> {
//     fn valid(&self) -> bool {
//         if ! self.is_empty() {
//             let mut last: &T = &self[0_usize];
//             for elem in &self[1_usize..] {
//                 if elem >= last {
//                     return false;
//                 }
//                 last = elem;
//             }
//         }
//         true
//     }

//     fn contains(&self, elem: &T) -> bool {
//         if ! self.is_empty() {
//             let mid = self.len() / 2;
//             if elem < &self[mid] {
//                 return self[..mid].contains(elem);
//             } else if &self[mid] < elem {
//                 return self[mid..].contains(elem);
//             } else {
//                 return true;
//             }
//         }
//         false
//     }

//     fn len(&self) -> usize { self.data.len() }
//     fn is_empty(&self) -> bool { self.data.is_empty() }
// }

struct AfuncList {
    data: SortedVec<usize>,
}

// impl AfuncList {
//     fn divides(&self, d: usize, n: usize) -> bool {
//         let divisors = &self.data[n];

//         assert!(divisors.valid());
//         // Remove for functions not a subset of division.
//         assert!(divisors[divisors.len()-1] < n);

//         return .contains(&d)
//     }
// }

// fn Afunc_D(usize n) -> AFuncList {
//     let mut vec = Vec<Vec<usize>>;
// }

fn main() {
    // let x: Vec<usize> = vec![0, 1, 2, 3, 4];
    // let mut y: Sorted<usize> = x;
    println!("Hello, world!");
}
