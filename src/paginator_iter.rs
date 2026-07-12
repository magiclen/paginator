use core::iter::FusedIterator;

use crate::{Paginator, YesNoDepends};

/// An iterator over paginator settings for consecutive current pages.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PaginatorIter {
    pub(crate) total_pages:    usize,
    pub(crate) current_page:   usize,
    pub(crate) back_page:      usize,
    pub(crate) max_item_count: usize,
    pub(crate) start_size:     usize,
    pub(crate) end_size:       usize,
    pub(crate) has_prev:       YesNoDepends,
    pub(crate) has_next:       YesNoDepends,
}

impl PaginatorIter {
    #[inline]
    fn to_page_config(&self, current_page: usize) -> Paginator {
        Paginator {
            total_pages: self.total_pages,
            current_page,
            max_item_count: self.max_item_count,
            start_size: self.start_size,
            end_size: self.end_size,
            has_prev: self.has_prev,
            has_next: self.has_next,
        }
    }
}

impl PaginatorIter {
    #[inline]
    fn remaining_pages(&self) -> usize {
        if self.current_page <= self.back_page {
            self.back_page - self.current_page + 1
        } else {
            0
        }
    }

    #[inline]
    fn exhaust(&mut self) {
        // Put the back cursor before the front cursor to mark the iterator as exhausted.
        self.back_page = self.current_page - 1;
    }
}

impl Iterator for PaginatorIter {
    type Item = Paginator;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_page <= self.back_page {
            let page_config = self.to_page_config(self.current_page);

            if self.current_page == self.back_page {
                self.exhaust();
            } else {
                self.current_page += 1;
            }

            Some(page_config)
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining_pages = self.remaining_pages();
        (remaining_pages, Some(remaining_pages))
    }

    #[inline]
    fn count(self) -> usize
    where
        Self: Sized, {
        self.remaining_pages()
    }

    #[inline]
    fn last(self) -> Option<Self::Item>
    where
        Self: Sized, {
        if self.current_page <= self.back_page {
            Some(self.to_page_config(self.back_page))
        } else {
            None
        }
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        if n < self.remaining_pages() {
            self.current_page += n;
            self.next()
        } else {
            self.exhaust();
            None
        }
    }
}

impl ExactSizeIterator for PaginatorIter {
    #[inline]
    fn len(&self) -> usize {
        self.remaining_pages()
    }
}

impl FusedIterator for PaginatorIter {}

impl DoubleEndedIterator for PaginatorIter {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.current_page <= self.back_page {
            let page_config = self.to_page_config(self.back_page);

            self.back_page -= 1;

            Some(page_config)
        } else {
            None
        }
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        if n < self.remaining_pages() {
            self.back_page -= n;
            self.next_back()
        } else {
            self.exhaust();
            None
        }
    }
}

// TODO ----------

impl Paginator {
    /// Iterate from the current page through the last page.
    #[inline]
    pub fn iter(&self) -> PaginatorIter {
        PaginatorIter {
            total_pages:    self.total_pages,
            current_page:   self.current_page,
            back_page:      self.total_pages,
            max_item_count: self.max_item_count,
            start_size:     self.start_size,
            end_size:       self.end_size,
            has_prev:       self.has_prev,
            has_next:       self.has_next,
        }
    }
}

impl IntoIterator for Paginator {
    type IntoIter = PaginatorIter;
    type Item = Paginator;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
