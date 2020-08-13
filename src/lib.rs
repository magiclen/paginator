/*!
# Paginator

This crate is used for generating pagination bar on webpages or other UIs.

## Examples

##### Creating a Pagination Bar Which Has 5 Pages and Is on Page 1

```rust
extern crate paginator;

use paginator::{Paginator, PageItem};

use core::fmt::Write;

let paginator = Paginator::builder(5).current_page(1).build_paginator().unwrap();

let mut html = String::new();

for page_item in paginator.paginate() {
    match page_item {
        PageItem::Prev(page) => {
            html.write_fmt(format_args!("<li><a href=\"/page/{page}\">&laquo;</a></li>", page = page)).unwrap();
        }
        PageItem::Page(page) => {
            html.write_fmt(format_args!("<li><a href=\"/page/{page}\">{page}</a></li>", page = page)).unwrap();
        }
        PageItem::CurrentPage(page) => {
            html.write_fmt(format_args!("<li>{page}</li>", page = page)).unwrap();
        }
        PageItem::Ignore => {
            html.push_str("<li>...</li>");
        }
        PageItem::Next(page) => {
            html.write_fmt(format_args!("<li><a href=\"/page/{page}\">&raquo;</a></li>", page = page)).unwrap();
        }
        _ => {
            // `PageItem::ReservedPrev` or `PageItem::ReservedNext` variants is used when the `has_prev` option or the `has_next` option is set to `YesNoDepends::Yes`.
        }
    }
}
```

##### Creating Pagination Bars Which Has 2 Pages for Different Current Pages

```rust
extern crate paginator;

use paginator::{Paginator, PageItem};

let mut paginator_iter = Paginator::builder(2).build_paginator_iter().unwrap();

for page_item in paginator_iter.next().unwrap().paginate() {
    // current_page == 1
}

for page_item in paginator_iter.next().unwrap().paginate() {
    // current_page == 2
}
```

## Pagination Rules

Before building up a `Paginator`, there is an important option, `max_item_count`, can be set via `PaginatorBuilder`. This option can limit the count of items on the pagination bar. This crate **ignores** page items far away from the current page item to stick to the count limit of items. The first/last n items can be additionally reserved.

Look at the following code for more details.

```rust
extern crate paginator;

use paginator::{Paginator, page_items_to_string};

let mut p = Paginator::builder(8).max_item_count(9).start_size(1).end_size(1).build_paginator_iter().unwrap();

assert_eq!("1* 2 3 4 5 6 7 8 >", page_items_to_string(p.next().unwrap().paginate().as_slice()));
assert_eq!("< 1 2* 3 4 5 ... 8 >", page_items_to_string(p.next().unwrap().paginate().as_slice()));
assert_eq!("< 1 2 3* 4 5 ... 8 >", page_items_to_string(p.next().unwrap().paginate().as_slice()));
assert_eq!("< 1 2 3 4* 5 ... 8 >", page_items_to_string(p.next().unwrap().paginate().as_slice()));
assert_eq!("< 1 ... 4 5* 6 7 8 >", page_items_to_string(p.next().unwrap().paginate().as_slice()));
assert_eq!("< 1 ... 4 5 6* 7 8 >", page_items_to_string(p.next().unwrap().paginate().as_slice()));
assert_eq!("< 1 ... 4 5 6 7* 8 >", page_items_to_string(p.next().unwrap().paginate().as_slice()));
assert_eq!("< 1 2 3 4 5 6 7 8*", page_items_to_string(p.next().unwrap().paginate().as_slice()));
```

## No Std

Disable the default features to compile this crate without std.

```toml
[dependencies.paginator]
version = "*"
default-features = false
```
*/

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

mod page_item;
mod paginator;
mod paginator_builder;
mod paginator_iter;
mod yes_no_depends;

use core::fmt::Write;

use alloc::string::String;

pub use self::paginator::*;
pub use page_item::*;
pub use paginator_builder::*;
pub use paginator_iter::*;
pub use yes_no_depends::*;

/// Format `PageItem`s to a string. Usually for debug or logging.
pub fn page_items_to_string(page_items: &[PageItem]) -> String {
    let mut s = String::new();

    let mut iter = page_items.iter();

    if let Some(item) = iter.next() {
        s.write_fmt(format_args!("{}", item)).unwrap();

        for item in iter {
            s.write_fmt(format_args!(" {}", item)).unwrap();
        }
    }

    s
}
