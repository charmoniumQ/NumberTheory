#![feature(plugin)]
#![plugin(clippy)]
#![feature(inclusive_range_syntax)]
//#![plugin(power_assert(override_builtins))]

use std::ops::Index;
use std::ops::Range;
use std::ops::RangeFrom;
use std::ops::RangeTo;
use std::vec::Vec;
use std::slice::Iter;
use std::clone::Clone;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Error;


pub struct SortedVec<T: PartialOrd> {
    // This struct 'inherits' from Vec, but hides some methods and adds some methods
    // Hidden methods are ones that might violate the "sorted-ness"
    // Thus, this type is compile-time guarunteed to be sorted,
    // following this pattern https://doc.rust-lang.org/book/second-edition/ch09-03-to-panic-or-not-to-panic.html#creating-custom-types-for-validation
    data: Vec<T>,
}

impl<T: PartialOrd + Debug> Debug for SortedVec<T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        self.data.fmt(f)
    }
}
impl<T: PartialOrd> Index<usize> for SortedVec<T> {
    type Output = T;
    fn index(&self, index: usize) -> &T { self.data.index(index) }
}
impl<T: PartialOrd> Index<Range<usize>> for SortedVec<T> {
    type Output = [T];
    fn index(&self, index: Range<usize>) -> &[T] { self.data.index(index) }
}
impl<T: PartialOrd> Index<RangeFrom<usize>> for SortedVec<T> {
    type Output = [T];
    fn index(&self, index: RangeFrom<usize>) -> &[T] { self.data.index(index) }
}
impl<T: PartialOrd> Index<RangeTo<usize>> for SortedVec<T> {
    type Output = [T];
    fn index(&self, index: RangeTo<usize>) -> &[T] { self.data.index(index) }
}

fn is_sorted<T: PartialOrd + Debug>(vec: &[T]) -> bool {
    if ! vec.is_empty() {
        let mut last: &T = &vec[0_usize];
        for elem in &vec[1_usize..] {
            if elem < last {
                return false;
            }
            last = elem;
        }
    }
    true
}

impl<T: PartialOrd + Debug> SortedVec<T> {
    pub fn new(source: Vec<T>) -> Option<SortedVec<T>> {
        if is_sorted(&source) {
            Some(SortedVec::<T> {data: source})
        } else {
            None
        }
    }

    pub fn contains(&self, elem: &T) -> bool {
        // for e2 in &self.data {
        //     if *e2 == *elem {
        //         return true;
        //     }
        // }
        // false
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

    pub fn len(&self) -> usize { self.data.len() }
    pub fn is_empty(&self) -> bool { self.data.is_empty() }
    pub fn iter(&self) -> Iter<T> { self.data.iter() }
    pub fn max(&self) -> Option<&T> {
        if self.is_empty() { None } else { Some(&self.data[self.len() - 1]) }
    }
}

pub fn intersection<T: PartialOrd + Debug + Clone>(a_vec: &SortedVec<T>, b_vec: &SortedVec<T>) -> SortedVec<T> {
    let mut result = Vec::<T>::new();
    let mut a_iter = a_vec.iter(); let mut b_iter = b_vec.iter();
    let mut a_elem = a_iter.next(); let mut b_elem = b_iter.next();
    while a_elem.is_some() && b_elem.is_some() {
        if a_elem > b_elem {
            b_elem = b_iter.next();
        } else if a_elem < b_elem {
            a_elem = a_iter.next();
        } else {
            result.push(a_elem.unwrap().clone());
            a_elem = a_iter.next();
            b_elem = b_iter.next();
        }
    }
    SortedVec::<T>::new(result).unwrap() // TODO: build this as a vec
}

pub fn is_intersection_empty<T: PartialOrd + Debug>(a_vec: &SortedVec<T>, b_vec: &SortedVec<T>) -> bool {
    let mut a_iter = a_vec.iter(); let mut b_iter = b_vec.iter();
    a_iter.next(); b_iter.next(); // TODO: do this differently
    let mut a_elem = a_iter.next(); let mut b_elem = b_iter.next();
    while a_elem.is_some() && b_elem.is_some() {
        if a_elem > b_elem {
            b_elem = b_iter.next();
        } else if a_elem < b_elem {
            a_elem = a_iter.next();
        } else {
            return false;
        }
    }
    true
}

pub struct AFunc {
    divisorss: Vec<SortedVec<usize>>,
}

fn valid_afunc(divisorss: &[SortedVec<usize>]) -> bool {
    divisorss.iter().enumerate().all(|(n, divisors)| {
        if ! divisors.is_empty() {
            *divisors.max().unwrap() <= n
        } else {
            true
        }
    })
}

impl AFunc {
    pub fn new(divisorss: Vec<SortedVec<usize>>) -> Option<AFunc> {
        if valid_afunc(&divisorss) {
            Some(AFunc {divisorss: divisorss})
        } else {
            None
        }
    }

    pub fn divides(&self, d: usize, n: usize) -> bool {
        self.divisorss[n].contains(&d)
    }

    pub fn d(n: usize) -> AFunc {
        AFunc::new((0..n).map(|i| {
            SortedVec::<usize>::new((0..=i).collect()).unwrap()
        }).collect()).unwrap()
    }

    pub fn gcd(&self, a: usize, b: usize) -> usize {
        *intersection(&self.divisorss[a], &self.divisorss[b]).max().unwrap()
    }

    pub fn coprime(&self, a: usize, b: usize) -> bool {
        is_intersection_empty(&self.divisorss[a], &self.divisorss[b])
    }

    pub fn iterate(&self) -> AFunc {
        AFunc::new((0..self.divisorss.len()).map(|n| {
            SortedVec::<usize>::new((0..=n).filter(|d| {
                self.coprime(*d, n-*d)
            }).collect()).unwrap()
        }).collect()).unwrap()
    }

    pub fn plaintext(&self) -> Vec<String> {
        self.divisorss.iter().enumerate().map(|(n, divisors)| {
            if ! divisors.is_empty() {
                let mut iter = divisors.iter();
                let mut possible_elem = iter.next();
                (0..=n).map(|d| {
                    if !possible_elem.is_some() || d < *possible_elem.unwrap() {
                        ' '
                    } else {
                        possible_elem = iter.next();
                        '*'
                    }
                }).collect()
            } else {
                String::new()
            }
        }).collect()
    }
}

fn main() {
    for line in AFunc::d(100).iterate().plaintext() {
        println!("{}", line);
    }
}
