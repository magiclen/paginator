use paginator::PaginatorBuilder;

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
