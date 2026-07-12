use paginator::{PaginatorBuildError, PaginatorBuilder, YesNoDepends};

#[test]
fn basic() {
    assert!(PaginatorBuilder::new(1).build_paginator().is_ok());
    assert!(PaginatorBuilder::new(1).build_paginator_iter().is_ok());
    assert!(PaginatorBuilder::new(0).current_page(1).build_paginator().is_err());
    assert!(PaginatorBuilder::new(1).current_page(0).build_paginator_iter().is_err());
    assert!(PaginatorBuilder::new(1).current_page(2).build_paginator_iter().is_err());
}

#[test]
fn forced_controls_on_small_page_counts() {
    let one_page = PaginatorBuilder::new(1).has_prev(YesNoDepends::Yes).has_next(YesNoDepends::Yes);

    assert_eq!(
        Err(PaginatorBuildError::MaxItemCountTooSmall {
            min_item_count: 3
        }),
        one_page.clone().max_item_count(2).build_paginator()
    );
    assert!(one_page.max_item_count(3).build_paginator().is_ok());

    let two_pages =
        PaginatorBuilder::new(2).has_prev(YesNoDepends::Yes).has_next(YesNoDepends::Yes);

    assert_eq!(
        Err(PaginatorBuildError::MaxItemCountTooSmall {
            min_item_count: 4
        }),
        two_pages.clone().max_item_count(3).build_paginator()
    );
    assert!(two_pages.max_item_count(4).build_paginator_iter().is_ok());
}

#[test]
fn max_item_count_1() {
    let builder = PaginatorBuilder::new(1)
        .start_size(1)
        .end_size(1)
        .has_prev(YesNoDepends::Depends)
        .has_next(YesNoDepends::Depends);

    assert!(builder.clone().total_pages(1).max_item_count(0).build_paginator().is_err());
    assert!(builder.clone().total_pages(1).max_item_count(1).build_paginator().is_ok());
    assert!(builder.clone().total_pages(2).max_item_count(1).build_paginator().is_err());
    assert!(builder.clone().total_pages(2).max_item_count(2).build_paginator().is_ok());
    assert!(builder.clone().total_pages(3).max_item_count(2).build_paginator().is_err());
    assert!(builder.clone().total_pages(3).max_item_count(3).build_paginator().is_err());
    assert!(builder.clone().total_pages(3).max_item_count(4).build_paginator().is_err());
    assert!(builder.clone().total_pages(3).max_item_count(5).build_paginator().is_ok());
    assert!(builder.clone().total_pages(4).max_item_count(5).build_paginator().is_err());
    assert!(builder.clone().total_pages(4).max_item_count(6).build_paginator().is_ok());
    assert!(builder.clone().total_pages(5).max_item_count(6).build_paginator().is_err());
    assert!(builder.clone().total_pages(5).max_item_count(7).build_paginator().is_ok());
    assert!(builder.clone().total_pages(6).max_item_count(7).build_paginator().is_ok());
    assert!(builder.clone().total_pages(7).max_item_count(7).build_paginator().is_ok());
    assert!(builder.total_pages(7).max_item_count(6).build_paginator().is_err());
}

#[test]
fn max_item_count_2() {
    let builder = PaginatorBuilder::new(1)
        .start_size(0)
        .end_size(0)
        .has_prev(YesNoDepends::No)
        .has_next(YesNoDepends::No);

    assert!(builder.clone().total_pages(1).max_item_count(0).build_paginator().is_err());
    assert!(builder.clone().total_pages(1).max_item_count(1).build_paginator().is_ok());
    assert!(builder.clone().total_pages(2).max_item_count(1).build_paginator().is_err());
    assert!(builder.clone().total_pages(2).max_item_count(2).build_paginator().is_ok());
    assert!(builder.clone().total_pages(3).max_item_count(2).build_paginator().is_err());
    assert!(builder.clone().total_pages(3).max_item_count(3).build_paginator().is_ok());
    assert!(builder.clone().total_pages(4).max_item_count(3).build_paginator().is_ok());
    assert!(builder.total_pages(4).max_item_count(2).build_paginator().is_err());
}

#[test]
fn max_item_count_3() {
    let builder = PaginatorBuilder::new(1)
        .start_size(2)
        .end_size(2)
        .has_prev(YesNoDepends::No)
        .has_next(YesNoDepends::No);

    assert!(builder.clone().total_pages(1).max_item_count(0).build_paginator().is_err());
    assert!(builder.clone().total_pages(1).max_item_count(1).build_paginator().is_ok());
    assert!(builder.clone().total_pages(2).max_item_count(1).build_paginator().is_err());
    assert!(builder.clone().total_pages(2).max_item_count(2).build_paginator().is_ok());
    assert!(builder.clone().total_pages(3).max_item_count(2).build_paginator().is_err());
    assert!(builder.clone().total_pages(3).max_item_count(3).build_paginator().is_ok());
    assert!(builder.clone().total_pages(4).max_item_count(3).build_paginator().is_err());
    assert!(builder.clone().total_pages(4).max_item_count(4).build_paginator().is_ok());
    assert!(builder.clone().total_pages(5).max_item_count(4).build_paginator().is_err());
    assert!(builder.clone().total_pages(5).max_item_count(5).build_paginator().is_ok());
    assert!(builder.clone().total_pages(6).max_item_count(5).build_paginator().is_err());
    assert!(builder.clone().total_pages(6).max_item_count(6).build_paginator().is_ok());
    assert!(builder.clone().total_pages(7).max_item_count(6).build_paginator().is_err());
    assert!(builder.clone().total_pages(7).max_item_count(7).build_paginator().is_ok());
    assert!(builder.clone().total_pages(8).max_item_count(7).build_paginator().is_ok());
    assert!(builder.total_pages(8).max_item_count(6).build_paginator().is_err());
}
