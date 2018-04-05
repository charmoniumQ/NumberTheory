use std::ops::{Index, Range, RangeInclusive, RangeFrom, RangeTo};
use std::vec::Vec;
use std::slice::Iter;
use std::clone::Clone;
use std::fmt::{Debug, Formatter};
use std::convert::{TryFrom, TryInto};
use std::iter::{Filter, Step};
use std::path::Path;
use image::{ImageBuffer, Rgb, RgbImage, Pixel};
use core::ops::DerefMut;
use std::cmp::min;
use scarlet::colormap::{GradientColorMap, NormalizeMapping, ColorMap};
use scarlet::color::RGBColor;
use std;

error_chain! { }

pub struct SortedVec<T: PartialOrd> {
    // This struct 'inherits' from Vec, but hides some methods and adds some methods
    // Hidden methods are ones that might violate the "sorted-ness"
    // Thus, objects of this type are compile-time guarunteed to be sorted,
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

#[allow(dead_code)]
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

#[allow(dead_code)]
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
        if divisors.len() == 0 {
            println!("{:?}", divisors);
            return false;
        }
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

#[allow(dead_code)]
impl AFunc {
    pub fn divides(&self, d: usize, n: usize) -> bool {
        self.divisorss[n].contains(&d)
    }

    // TODO: multiprocessed map
    // pub fn map<B>(self, f: FnMut<SortedVec<usize> -> B>) -> Vec<B> {
    //     let mut interm: Vec<Vec<SortedVec<usize>>> = Vec::new();
    //     for i in 0..(self.divisorss.len() / 2 + 1) {
    //         interm[i].push()
    //     }
    // }

    pub fn d(n: usize) -> AFunc {
        (0..n).map(|i| {
            (0..=i).into()
        }).collect::<Vec<SortedVec<usize>>>().try_into().unwrap()
        // since the ith divisor set is a subsequence of the range (0..=i),
        // the unwrapping succeeds
        // TODO: filtering on a range always works
    }

    // TODO: use Vaughn's algorithm here
    pub fn kary(k: usize, size: usize) -> AFunc {
        let div = AFunc::d(size);
        div.iterate_m(k)
    }


    pub fn gcd(&self, a: usize, b: usize) -> usize {
        // since the intersection contains at least 0, the unwrapping succeeds
        *intersection(&self.divisorss[a], &self.divisorss[b]).max().unwrap()
    }

    pub fn coprime(&self, a: usize, b: usize) -> bool {
        is_intersection_empty(&self.divisorss[a], &self.divisorss[b])
    }

    pub fn iterate_m(self, k: usize) -> AFunc {
        // TODO: take only a reference to self
        let mut div = self;
        for _ in 0..k {
            div = div.iterate();
        }
        div
    }

    pub fn iterate(&self) -> AFunc {
        (0..self.divisorss.len()).map(|n| -> SortedVec<usize> {
            (0..=n).filter(|d| {
                self.coprime(*d, n-*d)
            }).into()
            // since the nth divisor-set is up to n, the unwrapping succeeds
        }).collect::<Vec<SortedVec<usize>>>().try_into().unwrap()
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

    pub fn len(&self) -> usize {
        self.divisorss.len()
    }

    pub fn draw_plaintext(&self) -> Vec<String> {
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

    pub fn draw_image(&self, dest: &Path) {
        let sq_size: u32 = 2;
        let n_rows = self.len();
        let n_cols = n_rows + 1;

        let mut image = RgbImage::new(n_cols as u32 * sq_size,
                                      n_rows as u32 * sq_size);
        for row in 0..n_rows {
            for col in 0..=row {
                let left = n_rows - row + 2*col;
                let color = to_color(self.divides(col, row));
                draw_rect(&mut image, color,
                          left as u32 * sq_size / 2,
                          row as u32 * sq_size,
                          sq_size, sq_size);
            }
        }
        image.save(dest).unwrap();
    }

    pub fn mu(&self) -> Vec<i16> {
        // k_mobius n k = 0 - (sum [k_mobius i k | i <- [0..n-1], k_divides i n k])
        // TODO: find way to make fixed size vector
        let mut ret: Vec<i16> = (0..self.len()).map(|_| { 0 }).collect();
        if ! ret.is_empty() {
            ret[0] = 1;
            for n in 1..self.len() {
                let val: i16 = (0..n).map(|i| -> i16 {
                    if self.divides(i, n) {
                        ret[i]
                    } else {
                        0
                    }
                }).sum();
                ret[n] = -val;
            }
        }
        ret
    }
}

fn to_color(val: bool) -> Rgb<u8> {
    if val {
        Rgb{data: [255, 0, 0]}
    } else {
        Rgb{data: [0, 0, 255]}
    }
}

fn draw_rect<P, C>(image: &mut ImageBuffer<P, C>, color: P,
                   x: u32, y: u32, width: u32, height: u32)
where
    P: Pixel + 'static,
    P::Subpixel: 'static,
    C: DerefMut<Target = [P::Subpixel]>
{
    for i in x..(x+width) {
        for j in y..(y+height) {
            image.put_pixel(i, j, color);
        }
    }
}

pub struct CharTri {
    elems: Vec<Vec<usize>>,
}

fn valid_chartri(elems: &Vec<Vec<usize>>) -> bool {
    for i in 0..elems.len() {
        if elems[i].len() != i+1 {
            return false
        } else {
            // we know elems[i].len() > 0, so unwrapping succeeds
            if i != 0 && *elems[i].iter().max().unwrap() > i {
                return false
            }
        }
    }
    return true
}

impl TryFrom<Vec<Vec<usize>>> for CharTri {
    type Error = Error;
    fn try_from(elems: Vec<Vec<usize>>) -> Result<CharTri> {
        if valid_chartri(&elems) {
            Ok(CharTri {elems: elems})
        } else {
            Err("Not valid Characteristic Triangle".into())
        }
    }
}

impl CharTri {
    pub fn kary(size: usize) -> CharTri{
        // TODO: allocate the right size ahead of time
        // TODO: allocate the right size ahead of time in other places too
        let mut chartri: CharTri =
            (0..size).map(|row| {
                (0..=row).map(|_i| {
                    0
                }).collect()
            }).collect::<Vec<Vec<usize>>>().try_into().unwrap();
        // verify by hand that this satisfies valid_chartri, so unwrapping succeeds

        if size != 0 {
            chartri.elems[0][0] = 1
        }

        for b in 1..size {
            chartri.elems[b][0] = 1;
            chartri.elems[b][b] = 1;
            for a in 1..b {
                if a & b == a { // p^a infinitarily divides p^b, so m is odd
                    let mut lb = 1;
                    let mut ub = if b % 2 == 0 {
                        b - 1
                    } else {
                        b
                    };
                    let mut m = 0;
                    while lb != ub {
                        m = (lb + ub + 2) / 4 * 2 - 1;
                        if chartri.k_div(a, b, m) {
                            ub = m;
                        } else {
                            lb = m + 2;
                        }
                    }
                    chartri.elems[b][a] = lb;
                } else {
                    let mut lb = 2;
                    let mut ub = if b % 2 == 1 {
                        b - 1
                    } else {
                        b
                    };
                    let mut m = 0;
                    while lb != ub {
                        m = (lb + ub) / 4 * 2;
                        if chartri.k_div(a, b, m) {
                            lb = m + 2;
                        } else {
                            ub = m;
                        }
                    }
                    chartri.elems[b][a] = lb;
                }
            }
        }

        if ! valid_chartri(&chartri.elems) {
            panic!("not valid chartri");
        }
        // Our paper provides a proof of correctness of Vaughn's algorithm,
        // which ensures invariant valid_chartri

        chartri
    }

    fn k_div(&self, a: usize, b: usize, k: usize) -> bool {
        if k == 0 {
            return true
        }
        for i in 1..=min(a, b-a) {
            if CharTri::order(self.elems[a  ][i], k-1) &&
               CharTri::order(self.elems[b-a][i], k-1) {
                return false
            }
        }
        return true
    }

    fn order(k1: usize, k2: usize) -> bool {
        if k1 % 2 == 0 {
            k2 % 2 == 0 && k1 >  k2
        } else {
            k2 % 2 == 0 || k1 <= k2
        }
    }

    pub fn afunc(&self, k: usize) -> AFunc {
        // TODO: enumerate
        (0..self.elems.len()).map(|row_n| -> SortedVec<usize> {
            (0..=row_n).filter(|i| {
                return self.k_div(*i, row_n, k)
            }).try_into().unwrap()
            // because this is a filtered sub-sequence of 0..=row_n,
            // it is sorted and unwrapping succeeds
        }).collect::<Vec<SortedVec<usize>>>().try_into().unwrap()
        // unwrapping succeeds if SortedVec unwrapping succeeds and 
        // it was a valid CharTri
    }

    pub fn draw_image(&self, dest: &Path) {
        let sq_size: u32 = 2;
        let n_rows = self.elems.len();
        let n_cols = n_rows + 1;

        let mut image = RgbImage::new(n_cols as u32 * sq_size,
                                      n_rows as u32 * sq_size);
        for row in 0..n_rows {
            for col in 0..=row {
                let left = n_rows - row + 2*col;
                let color = to_color2(self.elems[row][col], self.elems.len());
                draw_rect(&mut image, color,
                          left as u32 * sq_size / 2,
                          row as u32 * sq_size,
                          sq_size, sq_size);
            }
        }
        let width = 10_u32;
        for i in 0..(n_rows / 10) {
            for j in 0..width {
                let color = to_color2(i * 10, self.elems.len());
                image.put_pixel(i as u32, j as u32, color);
            }
        }
        for i in 0..(n_rows / 10) {
            for j in 0..width {
                let color = to_color2(i * 10+1, self.elems.len());
                image.put_pixel(i as u32, width + 4 + j as u32, color);
            }
        }
        image.save(dest).unwrap();
    }
}

fn norm(v: f64) -> f64 { v.powf(0.7_f64) }

fn to_color2(val: usize, max: usize) -> Rgb<u8> {
    let odd_color = GradientColorMap::<RGBColor> {
        start: RGBColor::from_hex_code("#FFD915").unwrap(),
        end  : RGBColor::from_hex_code("#FF0D10").unwrap(),
        normalization: NormalizeMapping::Generic(norm),
        padding: (0_f64, 1_f64),
    };
    let even_color = GradientColorMap::<RGBColor> {
        start: RGBColor::from_hex_code("#12FFD0").unwrap(),
        end  : RGBColor::from_hex_code("#4E15FF").unwrap(),
        normalization: NormalizeMapping::Generic(norm),
        padding: (0_f64, 1_f64),
    };
    let color =  if val % 2 == 0 {
        // as f64
        even_color.transform_single((val as f64) / (max as f64))
    } else {
        odd_color .transform_single((val as f64) / (max as f64))
    };
    Rgb {data: [color.int_r(), color.int_g(), color.int_b()]}
}

// TODO: rethink the module subdivision
