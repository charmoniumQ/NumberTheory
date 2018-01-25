use std::ops::Index;
use std::ops::Range;
use std::ops::RangeInclusive;
use std::ops::RangeFrom;
use std::ops::RangeTo;
use std::vec::Vec;
use std::slice::Iter;
use std::clone::Clone;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::iter::Filter;
use std::iter::Step;

use std;

error_chain! { }

pub struct SortedVec<T: PartialOrd> {
    // This struct 'inherits' from Vec, but hides some methods and adds some methods
    // Hidden methods are ones that might violate the "sorted-ness"
    // Thus, this type is compile-time guarunteed to be sorted,
    // following this pattern https://doc.rust-lang.org/book/second-edition/ch09-03-to-panic-or-not-to-panic.html#creating-custom-types-for-validation
    data: Vec<T>,
}

impl<T: PartialOrd + Debug> Debug for SortedVec<T> {
    fn fmt(&self, f: &mut Formatter) -> std::result::Result<(), std::fmt::Error> {
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

fn is_sorted<T: PartialOrd>(vec: &[T]) -> bool {
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

impl<T: PartialOrd> TryFrom<Vec<T>> for SortedVec<T> {
    type Error = Error;
    fn try_from(source: Vec<T>) -> Result<SortedVec<T>> {
        if is_sorted(&source) {
            Ok(SortedVec::<T> {data: source})
        } else {
            Err("Not sorted".into())
        }
    }
}
impl<T: Step> From<RangeInclusive<T>> for SortedVec<T> {
    fn from(source: RangeInclusive<T>) -> SortedVec<T> {
        SortedVec::<T> {data: source.collect()}
    }
}
impl<T: Step, P: FnMut(&T) -> bool> From<Filter<RangeInclusive<T>, P>> for SortedVec<T> {
    fn from(source: Filter<RangeInclusive<T>, P>) -> SortedVec<T> {
        SortedVec::<T> {data: source.collect()}
    }
}

impl<T: PartialOrd> SortedVec<T> {
    pub fn new() -> SortedVec<T> {
        SortedVec::<T> {data: Vec::<T>::new()}
    }

    pub fn append(&mut self, elem: T) {
        if self.data[self.data.len() - 1] < elem{
            self.data.push(elem);
        } else {
            panic!("Not sorted");
        }
    }

    pub fn prepend(&mut self, elem: T) {
        if elem < self.data[self.data.len() - 1] {
            self.data.insert(0, elem);
        } else {
            panic!("Not sorted");
        }
    }

    pub fn contains(&self, elem: &T) -> bool {
        if ! self.is_empty() {
            let mid = self.len() / 2;
            if elem < &self[mid] {
                return self[..mid].contains(elem);
            } else if &self[mid] < elem {
                return self[mid+1..].contains(elem);
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

pub fn intersection<T: PartialOrd + Clone>(a_vec: &SortedVec<T>, b_vec: &SortedVec<T>) -> SortedVec<T> {
    let mut ret = SortedVec::<T>::new();
    let mut a_iter = a_vec.iter(); let mut b_iter = b_vec.iter();
    let mut a_result = a_iter.next(); let mut b_result = b_iter.next();
    loop {
        if let Some(a_elem) = a_result {
            if let Some(b_elem) = b_result {
                if a_elem > b_elem {
                    b_result = b_iter.next();
                } else if a_elem < b_elem {
                    a_result = a_iter.next();
                } else {
                    ret.append(a_elem.clone());
                    a_result = a_iter.next();
                    b_result = b_iter.next();
                }
            } else {
                break
            }
        } else {
            break
        }
    }
    ret.into()
}

pub fn is_intersection_empty<T: PartialOrd>(a_vec: &SortedVec<T>, b_vec: &SortedVec<T>) -> bool {
    let mut a_iter = a_vec.iter(); let mut b_iter = b_vec.iter();
    a_iter.next(); b_iter.next(); // TODO: do this differently
    let mut a_result = a_iter.next(); let mut b_result = b_iter.next();
    loop {
        if let Some(a_elem) = a_result {
            if let Some(b_elem) = b_result {
                if a_elem > b_elem {
                    b_result = b_iter.next();
                } else if a_elem < b_elem {
                    a_result = a_iter.next();
                } else {
                    return false;
                }
            } else {
                return true;
            }
        } else {
            return true;
        }
    }
}

pub struct AFunc {
    divisorss: Vec<SortedVec<usize>>,
}

fn valid_afunc(divisorss: &[SortedVec<usize>]) -> bool {
    divisorss.iter().enumerate().all(|(n, divisors)| {
        divisors[0] == 0 && match divisors.max() {
            Some(k) => *k <= n,
            None => true
        }
    })
}

impl TryFrom<Vec<SortedVec<usize>>> for AFunc {
    type Error = Error;
    fn try_from(divisorss: Vec<SortedVec<usize>>) -> Result<AFunc> {
        if valid_afunc(&divisorss) {
            Ok(AFunc {divisorss: divisorss})
        } else {
            Err("Not valid A-function".into())
        }
    }
}

impl AFunc {
    pub fn new() -> AFunc {
        AFunc {divisorss: Vec::<SortedVec<usize>>::new()}
    }

    pub fn divides(&self, d: usize, n: usize) -> bool {
        self.divisorss[n].contains(&d)
    }

    pub fn d(n: usize) -> AFunc {
        (0..n).map(|i| {
            (0..=i).into()
        }).collect::<Vec<SortedVec<usize>>>().try_into().unwrap()
        // since the ith divisor set is a subsequence of the range (0..=i),
        // the unwrapping succeeds
    }

    pub fn gcd(&self, a: usize, b: usize) -> usize {
        // since the intersection contains at least 0, the unwrapping succeeds
        *intersection(&self.divisorss[a], &self.divisorss[b]).max().unwrap()
    }

    pub fn coprime(&self, a: usize, b: usize) -> bool {
        is_intersection_empty(&self.divisorss[a], &self.divisorss[b])
    }

    pub fn iterate(&self) -> AFunc {
        (0..self.divisorss.len()).map(|n| -> SortedVec<usize> {
            (0..=n).filter(|d| {
                self.coprime(*d, n-*d)
            }).into()
            // since the nth divisor-set is up to n, the unwrapping succeeds
        }).collect::<Vec<SortedVec<usize>>>().try_into().unwrap()

    }

    pub fn plaintext(&self) -> Vec<String> {
        self.divisorss.iter().enumerate().map(|(n, divisors)| {
            if ! divisors.is_empty() {
                let mut iter = divisors.iter();
                let mut possible_elem = iter.next();
                (0..=n).map(|d| {
                    // since short-circuit logic, the unwrapping succeeds
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


    pub fn to_string(&self) -> String {
        self.divisorss.iter().map(|divisors| {
            divisors.iter().map(|divisor| {
                divisor.to_string()
            }).collect::<Vec<String>>().join(",")
        }).collect::<Vec<String>>().join("\n")
    }

    pub fn from_string(value: &String) -> Result<Self> {
        Ok(value.lines().map(|line| -> Result<SortedVec<usize>> {
            Ok(line.split(',').map(|num| {
                num.parse().chain_err(|| "can't parse num")
            }).collect::<Result<Vec<usize>>>()?.try_into()?)
        }).collect::<Result<Vec<SortedVec<usize>>>>()?.try_into()?)
    }
}
