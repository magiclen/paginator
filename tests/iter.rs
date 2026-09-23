use core::iter::FusedIterator;

use paginator::{Paginator, PaginatorBuilder};

fn assert_iterator_traits<T: ExactSizeIterator + FusedIterator>() {}

#[test]
fn one_page() {
    let mut iter = PaginatorBuilder::new(1).build_paginator_iter().unwrap();

    assert_eq!(1, iter.next().unwrap().current_page());
    assert_eq!(None, iter.next());
}

#[test]
fn two_pages() {
    let mut iter = PaginatorBuilder::new(2).build_paginator_iter().unwrap();

    assert_eq!(1, iter.next().unwrap().current_page());
    assert_eq!(2, iter.next().unwrap().current_page());
    assert_eq!(None, iter.next());
}

#[test]
fn two_pages_rev() {
    let mut iter = PaginatorBuilder::new(2).build_paginator_iter().unwrap().rev();

    assert_eq!(2, iter.next().unwrap().current_page());
    assert_eq!(1, iter.next().unwrap().current_page());
    assert_eq!(None, iter.next());
}

#[test]
fn exact_size_and_fused() {
    assert_iterator_traits::<paginator::PaginatorIter>();

    let mut iter = PaginatorBuilder::new(2).build_paginator_iter().unwrap();

    assert_eq!(2, iter.len());
    assert_eq!(1, iter.next().unwrap().current_page());
    assert_eq!(1, iter.len());
    assert_eq!(2, iter.next_back().unwrap().current_page());
    assert_eq!(0, iter.len());
    assert_eq!(None, iter.next());
    assert_eq!(None, iter.next_back());
}

#[test]
fn nth_and_nth_back() {
    let mut iter = PaginatorBuilder::new(6).build_paginator_iter().unwrap();

    assert_eq!(2, iter.nth(1).unwrap().current_page());
    assert_eq!(5, iter.nth_back(1).unwrap().current_page());
    assert_eq!(vec![3, 4], iter.map(|p| p.current_page()).collect::<Vec<_>>());
}

#[test]
fn iter_from_current_page() {
    let paginator = Paginator::builder(5).current_page(3).build_paginator().unwrap();

    assert_eq!(vec![3, 4, 5], paginator.iter().map(|p| p.current_page()).collect::<Vec<_>>());
    assert_eq!(vec![3, 4, 5], paginator.into_iter().map(|p| p.current_page()).collect::<Vec<_>>());
}

#[test]
fn large_indices_do_not_overflow() {
    let mut iter = PaginatorBuilder::new(2).build_paginator_iter().unwrap();

    assert_eq!(None, iter.nth(usize::MAX));
    assert_eq!(None, iter.next());

    let iter = PaginatorBuilder::new(usize::MAX).build_paginator_iter().unwrap();

    assert_eq!((usize::MAX, Some(usize::MAX)), iter.size_hint());
    assert_eq!(usize::MAX, iter.clone().count());
    assert_eq!(usize::MAX, iter.last().unwrap().current_page());

    let mut last_page =
        PaginatorBuilder::new(usize::MAX).current_page(usize::MAX).build_paginator_iter().unwrap();

    assert_eq!(usize::MAX, last_page.next().unwrap().current_page());
    assert_eq!(None, last_page.next());
}
