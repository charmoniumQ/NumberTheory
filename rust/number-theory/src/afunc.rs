use std::ops::Index;
use std::ops::Range;
use std::ops::RangeFrom;
use std::ops::RangeTo;
use std::vec::Vec;
use std::slice::Iter;
use std::clone::Clone;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::convert::TryFrom;
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
    pub fn new(source: Vec<T>) -> Result<SortedVec<T>, > {
        if is_sorted(&source) {
            Ok(SortedVec::<T> {data: source})
        } else {
            Err("Not sorted".into())
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

pub fn intersection<T: PartialOrd + Debug + Clone>(a_vec: &SortedVec<T>, b_vec: &SortedVec<T>) -> SortedVec<T> {
    let mut ret = Vec::<T>::new();
    let mut a_iter = a_vec.iter(); let mut b_iter = b_vec.iter();
    let mut a_elem = a_iter.next(); let mut b_elem = b_iter.next();
    while a_elem.is_some() && b_elem.is_some() {
        if a_elem > b_elem {
            b_elem = b_iter.next();
        } else if a_elem < b_elem {
            a_elem = a_iter.next();
        } else {
            ret.push(a_elem.unwrap().clone());
            a_elem = a_iter.next();
            b_elem = b_iter.next();
        }
    }
    // ret is sorted so unwrapping succeeds
    SortedVec::<T>::new(ret).unwrap() // TODO: build this as a vec
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
            // since divisors is not empty, unwrapping succeeds
            *divisors.max().unwrap() <= n
        } else {
            true
        }
    })
}

impl AFunc {
    pub fn new(divisorss: Vec<SortedVec<usize>>) -> Result<AFunc> {
        if valid_afunc(&divisorss) {
            Ok(AFunc {divisorss: divisorss})
        } else {
            Err("Not valid A-function".into())
        }
    }

    pub fn divides(&self, d: usize, n: usize) -> bool {
        self.divisorss[n].contains(&d)
    }

    pub fn d(n: usize) -> AFunc {
        AFunc::new((0..n).map(|i| {
            // since the vector is sorted, unwrapping succeeds
            SortedVec::<usize>::new((0..=i).collect()).unwrap()
        }).collect()).unwrap()
        // since the divisor-sets are valid range, the unwrapping succeeds
    }

    pub fn gcd(&self, a: usize, b: usize) -> usize {
        // since the intersection contains at least 0, the unwrapping succeeds
        *intersection(&self.divisorss[a], &self.divisorss[b]).max().unwrap()
    }

    pub fn coprime(&self, a: usize, b: usize) -> bool {
        is_intersection_empty(&self.divisorss[a], &self.divisorss[b])
    }

    pub fn iterate(&self) -> AFunc {
        AFunc::new((0..self.divisorss.len()).map(|n| {
            // since the new vector is sorted, the unwrapping succeeds
            SortedVec::<usize>::new((0..=n).filter(|d| {
                self.coprime(*d, n-*d)
            }).collect()).unwrap()
            // since the divisor-set is up to n, the unwrapping succeeds
        }).collect()).unwrap()

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
}

impl TryFrom<String> for AFunc {
    type Error = Error;
    fn try_from(value: String) -> Result<Self> {
        Ok(AFunc::new(
            value.lines().map(|line| -> Result<SortedVec<usize>> {
                Ok(SortedVec::new(line.split(',').map(|num| -> Result<usize> {
                    num.parse().chain_err(|| "can't parse num")
                }).collect::<Result<Vec<usize>>>()?)?)
            }).collect::<Result<Vec<SortedVec<usize>>>>()?
        )?)
    }
}

impl Into<String> for AFunc {
    fn into(self) -> String {
        self.divisorss.iter().map(|divisors| -> String {
            divisors.iter().map(|divisor| -> String {
                divisor.to_string()
            }).collect::<Vec<String>>().join(",")
        }).collect::<Vec<String>>().join("\n")
    }
}
