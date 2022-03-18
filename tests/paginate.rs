use paginator::{page_items_to_string, Paginator};

#[test]
fn one_page() {
    let mut p = Paginator::builder(1)
        .max_item_count(9)
        .start_size(1)
        .end_size(1)
        .build_paginator_iter()
        .unwrap();

    assert_eq!("1*", page_items_to_string(p.next().unwrap().paginate().as_slice()));
}

#[test]
fn two_pages() {
    let mut p = Paginator::builder(2)
        .max_item_count(9)
        .start_size(1)
        .end_size(1)
        .build_paginator_iter()
        .unwrap();

    assert_eq!("1* 2", page_items_to_string(p.next().unwrap().paginate().as_slice()));
    assert_eq!("1 2*", page_items_to_string(p.next().unwrap().paginate().as_slice()));
}

#[test]
fn three_pages() {
    let mut p = Paginator::builder(3)
        .max_item_count(9)
        .start_size(1)
        .end_size(1)
        .build_paginator_iter()
        .unwrap();

    assert_eq!("1* 2 3 >", page_items_to_string(p.next().unwrap().paginate().as_slice()));
    assert_eq!("< 1 2* 3 >", page_items_to_string(p.next().unwrap().paginate().as_slice()));
    assert_eq!("< 1 2 3*", page_items_to_string(p.next().unwrap().paginate().as_slice()));
}

#[test]
fn four_pages() {
    let mut p = Paginator::builder(4)
        .max_item_count(9)
        .start_size(1)
        .end_size(1)
        .build_paginator_iter()
        .unwrap();

    assert_eq!("1* 2 3 4 >", page_items_to_string(p.next().unwrap().paginate().as_slice()));
    assert_eq!("< 1 2* 3 4 >", page_items_to_string(p.next().unwrap().paginate().as_slice()));
    assert_eq!("< 1 2 3* 4 >", page_items_to_string(p.next().unwrap().paginate().as_slice()));
    assert_eq!("< 1 2 3 4*", page_items_to_string(p.next().unwrap().paginate().as_slice()));
}

#[test]
fn five_pages() {
    let mut p = Paginator::builder(5)
        .max_item_count(9)
        .start_size(1)
        .end_size(1)
        .build_paginator_iter()
        .unwrap();

    assert_eq!("1* 2 3 4 5 >", page_items_to_string(p.next().unwrap().paginate().as_slice()));
    assert_eq!("< 1 2* 3 4 5 >", page_items_to_string(p.next().unwrap().paginate().as_slice()));
    assert_eq!("< 1 2 3* 4 5 >", page_items_to_string(p.next().unwrap().paginate().as_slice()));
    assert_eq!("< 1 2 3 4* 5 >", page_items_to_string(p.next().unwrap().paginate().as_slice()));
    assert_eq!("< 1 2 3 4 5*", page_items_to_string(p.next().unwrap().paginate().as_slice()));
}

#[test]
fn six_pages() {
    let mut p = Paginator::builder(6)
        .max_item_count(9)
        .start_size(1)
        .end_size(1)
        .build_paginator_iter()
        .unwrap();

    assert_eq!("1* 2 3 4 5 6 >", page_items_to_string(p.next().unwrap().paginate().as_slice()));
    assert_eq!("< 1 2* 3 4 5 6 >", page_items_to_string(p.next().unwrap().paginate().as_slice()));
    assert_eq!("< 1 2 3* 4 5 6 >", page_items_to_string(p.next().unwrap().paginate().as_slice()));
    assert_eq!("< 1 2 3 4* 5 6 >", page_items_to_string(p.next().unwrap().paginate().as_slice()));
    assert_eq!("< 1 2 3 4 5* 6 >", page_items_to_string(p.next().unwrap().paginate().as_slice()));
    assert_eq!("< 1 2 3 4 5 6*", page_items_to_string(p.next().unwrap().paginate().as_slice()));
}

#[test]
fn seven_pages() {
    let mut p = Paginator::builder(7)
        .max_item_count(9)
        .start_size(1)
        .end_size(1)
        .build_paginator_iter()
        .unwrap();

    assert_eq!("1* 2 3 4 5 6 7 >", page_items_to_string(p.next().unwrap().paginate().as_slice()));
    assert_eq!("< 1 2* 3 4 5 6 7 >", page_items_to_string(p.next().unwrap().paginate().as_slice()));
    assert_eq!("< 1 2 3* 4 5 6 7 >", page_items_to_string(p.next().unwrap().paginate().as_slice()));
    assert_eq!("< 1 2 3 4* 5 6 7 >", page_items_to_string(p.next().unwrap().paginate().as_slice()));
    assert_eq!("< 1 2 3 4 5* 6 7 >", page_items_to_string(p.next().unwrap().paginate().as_slice()));
    assert_eq!("< 1 2 3 4 5 6* 7 >", page_items_to_string(p.next().unwrap().paginate().as_slice()));
    assert_eq!("< 1 2 3 4 5 6 7*", page_items_to_string(p.next().unwrap().paginate().as_slice()));
}

#[test]
fn eight_pages() {
    let mut p = Paginator::builder(8)
        .max_item_count(9)
        .start_size(1)
        .end_size(1)
        .build_paginator_iter()
        .unwrap();

    assert_eq!("1* 2 3 4 5 6 7 8 >", page_items_to_string(p.next().unwrap().paginate().as_slice()));
    assert_eq!(
        "< 1 2* 3 4 5 ... 8 >",
        page_items_to_string(p.next().unwrap().paginate().as_slice())
    );
    assert_eq!(
        "< 1 2 3* 4 5 ... 8 >",
        page_items_to_string(p.next().unwrap().paginate().as_slice())
    );
    assert_eq!(
        "< 1 2 3 4* 5 ... 8 >",
        page_items_to_string(p.next().unwrap().paginate().as_slice())
    );
    assert_eq!(
        "< 1 ... 4 5* 6 7 8 >",
        page_items_to_string(p.next().unwrap().paginate().as_slice())
    );
    assert_eq!(
        "< 1 ... 4 5 6* 7 8 >",
        page_items_to_string(p.next().unwrap().paginate().as_slice())
    );
    assert_eq!(
        "< 1 ... 4 5 6 7* 8 >",
        page_items_to_string(p.next().unwrap().paginate().as_slice())
    );
    assert_eq!("< 1 2 3 4 5 6 7 8*", page_items_to_string(p.next().unwrap().paginate().as_slice()));
}

#[test]
fn nine_pages() {
    let mut p = Paginator::builder(9)
        .max_item_count(9)
        .start_size(1)
        .end_size(1)
        .build_paginator_iter()
        .unwrap();

    assert_eq!(
        "1* 2 3 4 5 6 ... 9 >",
        page_items_to_string(p.next().unwrap().paginate().as_slice())
    );
    assert_eq!(
        "< 1 2* 3 4 5 ... 9 >",
        page_items_to_string(p.next().unwrap().paginate().as_slice())
    );
    assert_eq!(
        "< 1 2 3* 4 5 ... 9 >",
        page_items_to_string(p.next().unwrap().paginate().as_slice())
    );
    assert_eq!(
        "< 1 2 3 4* 5 ... 9 >",
        page_items_to_string(p.next().unwrap().paginate().as_slice())
    );
    assert_eq!(
        "< 1 ... 4 5* 6 ... 9 >",
        page_items_to_string(p.next().unwrap().paginate().as_slice())
    );
    assert_eq!(
        "< 1 ... 5 6* 7 8 9 >",
        page_items_to_string(p.next().unwrap().paginate().as_slice())
    );
    assert_eq!(
        "< 1 ... 5 6 7* 8 9 >",
        page_items_to_string(p.next().unwrap().paginate().as_slice())
    );
    assert_eq!(
        "< 1 ... 5 6 7 8* 9 >",
        page_items_to_string(p.next().unwrap().paginate().as_slice())
    );
    assert_eq!(
        "< 1 ... 4 5 6 7 8 9*",
        page_items_to_string(p.next().unwrap().paginate().as_slice())
    );
}
