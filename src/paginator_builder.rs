use core::fmt::{self, Display, Formatter};

#[cfg(feature = "std")]
use std::error::Error;

use crate::{Paginator, PaginatorIter};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PaginatorBuildError {
    CurrentPageZero,
    TotalPagesZero,
    CurrentPageTooLarge {
        current_page: usize,
        total_pages: usize,
    },
    MaxItemCountTooSmall {
        min_item_count: usize,
    },
}

impl Display for PaginatorBuildError {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            PaginatorBuildError::CurrentPageZero => f.write_str("current_page should not be zero"),
            PaginatorBuildError::TotalPagesZero => f.write_str("total_pages should not be zero"),
            PaginatorBuildError::CurrentPageTooLarge {
                current_page,
                total_pages,
            } => {
                f.write_fmt(format_args!(
                    "{current_page} > {total_pages} (current_page > total_pages)",
                    current_page = current_page,
                    total_pages = total_pages
                ))
            }
            PaginatorBuildError::MaxItemCountTooSmall {
                min_item_count,
            } => {
                f.write_fmt(format_args!(
                    "max_item_count cannot be smaller than {}",
                    min_item_count
                ))
            }
        }
    }
}

#[cfg(feature = "std")]
impl Error for PaginatorBuildError {}

/// A struct to create `Paginator` or `PaginatorIter`.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PaginatorBuilder {
    /// The number of pages.
    pub total_pages: usize,
    /// The number of the current page.
    pub current_page: usize,
    /// The max number of `PageItem`s after generated.
    pub max_item_count: usize,
    /// The number of `PageItem`s (the `PageItem::Prev` item is excluded) on the start edge (before the first `PageItem::Ignore` item).
    pub start_size: usize,
    /// The number of `PageItem`s (the `PageItem::Next` item is excluded) on the end edge (after the last `PageItem::Ignore` item).
    pub end_size: usize,
    /// Whether to add the `PageItem::Prev` item.
    pub has_prev: bool,
    /// Whether to add the `PageItem::Next` item.
    pub has_next: bool,
}

impl PaginatorBuilder {
    /**
        Create a new `PageConfigBuilder` with some default options.

        ```text
        PageConfigBuilder {
            total_pages: <total_pages>,
            current_page: 1,
            max_item_count: 9,
            start_size: 1,
            end_size: 1,
            has_prev: true,
            has_next: true,
        }
        ```
    */
    #[inline]
    pub const fn new(total_pages: usize) -> PaginatorBuilder {
        PaginatorBuilder {
            total_pages,
            current_page: 1,
            max_item_count: 9,
            start_size: 1,
            end_size: 1,
            has_prev: true,
            has_next: true,
        }
    }

    /// Set the number of pages.
    #[inline]
    pub const fn total_pages(mut self, total_pages: usize) -> PaginatorBuilder {
        self.total_pages = total_pages;

        self
    }

    /// Set the number of the current page.
    #[inline]
    pub const fn current_page(mut self, current_page: usize) -> PaginatorBuilder {
        self.current_page = current_page;

        self
    }

    /// Set the max number of `PageItem`s after generated.
    #[inline]
    pub const fn max_item_count(mut self, max_item_count: usize) -> PaginatorBuilder {
        self.max_item_count = max_item_count;

        self
    }

    /// Set the number of `PageItem`s (the `PageItem::Prev` item is excluded) on the start edge (before the first `PageItem::Ignore` item).
    #[inline]
    pub const fn start_size(mut self, start_size: usize) -> PaginatorBuilder {
        self.start_size = start_size;

        self
    }

    /// Set the number of `PageItem`s (the `PageItem::Next` item is excluded) on the end edge (after the last `PageItem::Ignore` item).
    #[inline]
    pub const fn end_size(mut self, end_size: usize) -> PaginatorBuilder {
        self.end_size = end_size;

        self
    }

    /// Set whether to add the `PageItem::Prev` item.
    #[inline]
    pub const fn has_prev(mut self, has_prev: bool) -> PaginatorBuilder {
        self.has_prev = has_prev;

        self
    }

    /// Set whether to add the `PageItem::Next` item.
    #[inline]
    pub const fn has_next(mut self, has_next: bool) -> PaginatorBuilder {
        self.has_next = has_next;

        self
    }
}

impl PaginatorBuilder {
    fn compute_min_item_count(&self) -> usize {
        match self.total_pages {
            0 => 0,
            1 => 1,
            2 => 2,
            _ => {
                let start_size = self.start_size.min(self.total_pages);
                let end_size = self.end_size.min(self.total_pages);
                let size = start_size + end_size;

                let mut min_item_count = (size + 3).min(self.total_pages);

                if self.has_prev {
                    min_item_count += 1;
                }

                if self.has_next {
                    min_item_count += 1;
                }

                min_item_count
            }
        }
    }

    #[inline]
    fn build_check_common(&self) -> Result<(), PaginatorBuildError> {
        if self.current_page == 0 {
            return Err(PaginatorBuildError::CurrentPageZero);
        }

        if self.total_pages == 0 {
            return Err(PaginatorBuildError::TotalPagesZero);
        }

        if self.current_page > self.total_pages {
            return Err(PaginatorBuildError::CurrentPageTooLarge {
                current_page: self.current_page,
                total_pages: self.total_pages,
            });
        }

        let min_item_count = self.compute_min_item_count();

        if self.max_item_count < min_item_count {
            return Err(PaginatorBuildError::MaxItemCountTooSmall {
                min_item_count,
            });
        }

        Ok(())
    }

    #[inline]
    pub fn build_paginator(self) -> Result<Paginator, PaginatorBuildError> {
        self.build_check_common()?;

        Ok(Paginator {
            total_pages: self.total_pages,
            current_page: self.current_page,
            max_item_count: self.max_item_count,
            start_size: self.start_size,
            end_size: self.end_size,
            has_prev: self.has_prev,
            has_next: self.has_next,
        })
    }

    #[inline]
    pub fn build_paginator_iter(self) -> Result<PaginatorIter, PaginatorBuildError> {
        self.build_check_common()?;

        Ok(PaginatorIter {
            total_pages: self.total_pages,
            current_page: self.current_page,
            back_page: self.total_pages,
            max_item_count: self.max_item_count,
            start_size: self.start_size,
            end_size: self.end_size,
            has_prev: self.has_prev,
            has_next: self.has_next,
        })
    }
}

// TODO ----------

impl Paginator {
    /// An alias of `PaginatorBuilder::new`.
    #[inline]
    pub fn builder(total_pages: usize) -> PaginatorBuilder {
        PaginatorBuilder::new(total_pages)
    }
}

impl PaginatorIter {
    /// An alias of `PaginatorBuilder::new`.
    #[inline]
    pub fn builder(total_pages: usize) -> PaginatorBuilder {
        PaginatorBuilder::new(total_pages)
    }
}
