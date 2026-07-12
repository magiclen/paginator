use core::iter::FusedIterator;

use paginator::PaginatorBuilder;

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
