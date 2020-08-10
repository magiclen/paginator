extern crate paginator;

use paginator::PaginatorBuilder;

#[test]
fn basic() {
    let mut iter = PaginatorBuilder::new(1).build_paginator_iter().unwrap();

    assert_eq!(1, iter.next().unwrap().current_page());
    assert_eq!(None, iter.next());
    assert_eq!(1, iter.next_back().unwrap().current_page());
    assert_eq!(None, iter.next_back());
    assert_eq!(1, iter.next().unwrap().current_page());
}
