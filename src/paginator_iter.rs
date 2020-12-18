use crate::{Paginator, YesNoDepends};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PaginatorIter {
    pub(crate) total_pages: usize,
    pub(crate) current_page: usize,
    pub(crate) back_page: usize,
    pub(crate) max_item_count: usize,
    pub(crate) start_size: usize,
    pub(crate) end_size: usize,
    pub(crate) has_prev: YesNoDepends,
    pub(crate) has_next: YesNoDepends,
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
    unsafe fn next_unchecked(&mut self) -> Paginator {
        let page_config = self.to_page_config(self.current_page);

        self.current_page += 1;

        page_config
    }

    #[inline]
    unsafe fn next_back_unchecked(&mut self) -> Paginator {
        let page_config = self.to_page_config(self.back_page);

        self.back_page -= 1;

        page_config
    }
}

impl Iterator for PaginatorIter {
    type Item = Paginator;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_page <= self.back_page {
            Some(unsafe { self.next_unchecked() })
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining_pages = self.back_page + 1 - self.current_page;
        (remaining_pages, Some(remaining_pages))
    }

    #[inline]
    fn count(self) -> usize
    where
        Self: Sized, {
        if self.current_page <= self.back_page {
            self.back_page + 1 - self.current_page
        } else {
            0
        }
    }

    #[inline]
    fn last(mut self) -> Option<Self::Item>
    where
        Self: Sized, {
        if self.current_page <= self.back_page {
            self.current_page = self.back_page;

            Some(unsafe { self.next_unchecked() })
        } else {
            None
        }
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.current_page += n;

        if self.current_page <= self.back_page {
            Some(unsafe { self.next_unchecked() })
        } else {
            self.current_page = self.back_page + 1;

            None
        }
    }
}

impl DoubleEndedIterator for PaginatorIter {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.current_page <= self.back_page {
            Some(unsafe { self.next_back_unchecked() })
        } else {
            None
        }
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        if self.back_page > n {
            self.back_page -= n;

            if self.current_page <= self.back_page {
                return Some(unsafe { self.next_back_unchecked() });
            }
        }

        self.back_page = self.current_page - 1;

        None
    }
}

// TODO ----------

impl Paginator {
    #[inline]
    pub fn iter(&self) -> PaginatorIter {
        PaginatorIter {
            total_pages: self.total_pages,
            current_page: self.current_page,
            back_page: self.total_pages,
            max_item_count: self.max_item_count,
            start_size: self.start_size,
            end_size: self.end_size,
            has_prev: self.has_prev,
            has_next: self.has_next,
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
