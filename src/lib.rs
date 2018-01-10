#![feature(test)]
extern crate num;
extern crate test;

use num::bigint::BigInt;

use std::mem::replace;
use std::collections::LinkedList;

pub fn derangements_i(mut i: i64) -> BigInt {
    match i {
        0 => BigInt::from(1),
        1 => BigInt::from(0),
        _ => {
            let mut n1 = BigInt::from(0);
            let mut n0 = BigInt::from(1);
            while i != 0 {
                let n2 = (i - 1) * (n0 + &n1);
                n0 = replace(&mut n1, n2);
                i = i - 1;
            }
            n0
        }
    }
}

pub fn derangements_u(mut i: u64) -> BigInt {
    match i {
        0 => BigInt::from(1),
        1 => BigInt::from(0),
        _ => {
            let mut n1 = BigInt::from(0);
            let mut n0 = BigInt::from(1);
            while i != 0 {
                let n2 = (i - 1) * (n0 + &n1);
                n0 = replace(&mut n1, n2);
                i = i - 1;
            }
            n0
        }
    }
}

pub fn derangements_fast(i: u32) -> BigInt {
    match i {
        0 => BigInt::from(1),
        1 => BigInt::from(0),
        _ => {
            let mut n1 = BigInt::from(0);
            let mut n0 = BigInt::from(1);
            let mut index = 1;
            while index < i {
                let n2 = (n0 + &n1) * index;
                n0 = replace(&mut n1, n2);
                index += 1;

            }
            n1
        }
    }
}

pub fn derangements_cached(x: usize) -> BigInt {
    static mut STATIC_CACHE: Option<LinkedList<(usize, BigInt)>> = None;
    unsafe {
        let cache = STATIC_CACHE.get_or_insert_with(cache_init);

        while cache.back().unwrap().0 < x {
            let new: BigInt;
            let index: usize;
            {
                let mut iter = cache.iter();
                let &(i, ref n1) = iter.next_back().unwrap();
                let &(_, ref n0) = iter.next_back().unwrap();
                index = i + 1;
                new = (n0 + n1) * i;
            }
            cache.push_back((index, new))   
        }

        cache.iter().find(|&&(i, _)| i == x).unwrap().1.clone()
        
    }
}

fn cache_init() -> LinkedList<(usize, BigInt)> {
    let mut cache = LinkedList::new();
    cache.push_back((0, BigInt::from(1)));
    cache.push_back((1, BigInt::from(0)));
    cache
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_derange_i(b: &mut Bencher) {
        b.iter(|| derangements_i(64));
    }

    #[bench]
    fn bench_derange_u(b: &mut Bencher) {
        b.iter(|| derangements_u(64));
    }

    #[bench]
    fn bench_derange_fast(b: &mut Bencher) {
        b.iter(|| derangements_fast(64));
    }

    #[bench]
    fn bench_derange_cached(b: &mut Bencher) {
        b.iter(|| derangements_cached(64));
    }

    #[test]
    fn test_derangements_fast() {
        let expected = BigInt::from(1334961);
        assert_eq!(derangements_fast(10), expected);
    }
    #[test]
    fn test_derangements_u() {
        let expected = BigInt::from(1334961);
        assert_eq!(derangements_u(10), expected);
    }
    #[test]
    fn test_derangements_i() {
        let expected = BigInt::from(1334961);
        assert_eq!(derangements_i(10), expected);
    }
    #[test]
    fn test_derangements_cached() {
        let expected = BigInt::from(1334961);
        assert_eq!(derangements_cached(10), expected);
    }
    
}
