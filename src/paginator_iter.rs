use crate::Paginator;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PaginatorIter {
    // next_page CANNOT be bigger than `total_pages + 1`
    pub(crate) total_pages: usize,
    pub(crate) next_page: usize,
    pub(crate) max_item_count: usize,
    pub(crate) start_size: usize,
    pub(crate) end_size: usize,
    pub(crate) has_prev: bool,
    pub(crate) has_next: bool,
}

impl PaginatorIter {
    #[inline]
    fn to_page_config(&self) -> Paginator {
        Paginator {
            total_pages: self.total_pages,
            current_page: self.next_page,
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
        let page_config = self.to_page_config();

        self.next_page += 1;

        page_config
    }
}

impl Iterator for PaginatorIter {
    type Item = Paginator;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.next_page <= self.total_pages {
            Some(unsafe { self.next_unchecked() })
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining_pages = self.total_pages - self.next_page + 1;

        (remaining_pages, Some(remaining_pages))
    }

    #[inline]
    fn count(mut self) -> usize
    where
        Self: Sized, {
        let remaining_pages = self.total_pages - self.next_page + 1;

        self.next_page = self.total_pages + 1;

        remaining_pages
    }

    #[inline]
    fn last(mut self) -> Option<Self::Item>
    where
        Self: Sized, {
        if self.total_pages == 0 {
            None
        } else {
            self.next_page = self.total_pages;

            Some(unsafe { self.next_unchecked() })
        }
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.next_page += n;

        if self.next_page > self.total_pages {
            self.next_page = self.total_pages;

            None
        } else {
            Some(unsafe { self.next_unchecked() })
        }
    }
}

impl DoubleEndedIterator for PaginatorIter {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.next_page > 1 && self.total_pages > 0 {
            self.next_page -= 1;

            Some(self.to_page_config())
        } else {
            None
        }
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        if self.next_page > n && self.total_pages > 0 {
            self.next_page -= n;

            Some(self.to_page_config())
        } else {
            None
        }
    }
}

// TODO ----------

impl Paginator {
    #[inline]
    pub fn iter(&self) -> PaginatorIter {
        PaginatorIter {
            total_pages: self.total_pages,
            next_page: self.current_page,
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
