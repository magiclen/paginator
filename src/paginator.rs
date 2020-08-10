use core::num::NonZeroUsize;

use alloc::vec::Vec;

use crate::PageItem;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Paginator {
    // total_pages and current_page is always bigger than 0 and current_page CANNOT be bigger than total_pages.
    pub(crate) total_pages: usize,
    pub(crate) current_page: usize,
    pub(crate) max_item_count: usize,
    pub(crate) start_size: usize,
    pub(crate) end_size: usize,
    pub(crate) has_prev: bool,
    pub(crate) has_next: bool,
}

impl Paginator {
    #[inline]
    pub const fn total_pages(&self) -> usize {
        self.total_pages
    }

    #[inline]
    pub const fn current_page(&self) -> usize {
        self.current_page
    }

    #[inline]
    pub const fn max_item_count(&self) -> usize {
        self.max_item_count
    }

    #[inline]
    pub const fn start_size(&self) -> usize {
        self.start_size
    }

    #[inline]
    pub const fn end_size(&self) -> usize {
        self.end_size
    }

    #[inline]
    pub const fn has_prev(&self) -> bool {
        self.has_prev
    }

    #[inline]
    pub const fn has_next(&self) -> bool {
        self.has_next
    }
}

impl Paginator {
    /// Create `PageItem`s.
    pub fn paginate(&self) -> Vec<PageItem> {
        let mut v = Vec::new();
        let mut items_counter = self.max_item_count;

        let show_prev = self.has_prev && self.current_page > 1 && self.total_pages > 2;
        let show_next =
            self.has_next && self.current_page < self.total_pages && self.total_pages > 2;

        if show_prev {
            v.push(PageItem::Prev(unsafe { NonZeroUsize::new_unchecked(self.current_page - 1) }));

            items_counter -= 1;
        }

        if show_next {
            items_counter -= 1;
        }

        let start_size = self.start_size.min(self.total_pages);
        let end_size = self.start_size.min(self.total_pages);

        let (ignore_start, ignore_end) = if self.total_pages > items_counter {
            let ignore_start = self.current_page >= 3 + start_size;
            let ignore_end = self.total_pages - self.current_page + 1 >= 3 + end_size;

            (ignore_start, ignore_end)
        } else {
            (false, false)
        };

        if ignore_start {
            if ignore_end {
                items_counter -= start_size + 2; // start_size + 1 + 1
                items_counter -= end_size + 1;

                for i in 1..=start_size {
                    v.push(PageItem::Page(unsafe { NonZeroUsize::new_unchecked(i) }));
                }

                let padding = items_counter >> 1;
                let padding_p = items_counter - padding;

                if start_size + 2 == self.current_page - padding {
                    v.push(PageItem::Page(unsafe { NonZeroUsize::new_unchecked(start_size + 1) }));
                } else {
                    v.push(PageItem::Ignore);
                }

                for i in (self.current_page - padding)..=(self.current_page - 1) {
                    v.push(PageItem::Page(unsafe { NonZeroUsize::new_unchecked(i) }));
                }

                v.push(PageItem::CurrentPage(unsafe {
                    NonZeroUsize::new_unchecked(self.current_page)
                }));

                for i in (self.current_page + 1)..=(self.current_page + padding_p) {
                    v.push(PageItem::Page(unsafe { NonZeroUsize::new_unchecked(i) }));
                }

                if self.current_page + padding_p + 2 == self.total_pages - self.end_size + 1 {
                    v.push(PageItem::Page(unsafe {
                        NonZeroUsize::new_unchecked(self.current_page + padding_p + 1)
                    }));
                } else {
                    v.push(PageItem::Ignore);
                }

                for i in (self.total_pages - self.end_size + 1)..=self.total_pages {
                    v.push(PageItem::Page(unsafe { NonZeroUsize::new_unchecked(i) }));
                }
            } else {
                items_counter -= start_size + 2; // start_size + 1 + 1

                if self.current_page < self.total_pages {
                    items_counter -= self.total_pages - self.current_page;
                }

                for i in 1..=start_size {
                    v.push(PageItem::Page(unsafe { NonZeroUsize::new_unchecked(i) }));
                }

                v.push(PageItem::Ignore);

                for i in (self.current_page - items_counter)..=(self.current_page - 1) {
                    v.push(PageItem::Page(unsafe { NonZeroUsize::new_unchecked(i) }));
                }

                v.push(PageItem::CurrentPage(unsafe {
                    NonZeroUsize::new_unchecked(self.current_page)
                }));

                for i in (self.current_page + 1)..=self.total_pages {
                    v.push(PageItem::Page(unsafe { NonZeroUsize::new_unchecked(i) }));
                }
            }
        } else if ignore_end {
            items_counter -= end_size + 1;

            items_counter -= self.current_page;

            for i in 1..self.current_page {
                v.push(PageItem::Page(unsafe { NonZeroUsize::new_unchecked(i) }));
            }

            v.push(PageItem::CurrentPage(unsafe {
                NonZeroUsize::new_unchecked(self.current_page)
            }));

            for i in (self.current_page + 1)..=(self.current_page + items_counter) {
                v.push(PageItem::Page(unsafe { NonZeroUsize::new_unchecked(i) }));
            }

            v.push(PageItem::Ignore);

            for i in (self.total_pages - self.end_size + 1)..=self.total_pages {
                v.push(PageItem::Page(unsafe { NonZeroUsize::new_unchecked(i) }));
            }
        } else {
            for i in 1..self.current_page {
                v.push(PageItem::Page(unsafe { NonZeroUsize::new_unchecked(i) }));
            }

            v.push(PageItem::CurrentPage(unsafe {
                NonZeroUsize::new_unchecked(self.current_page)
            }));

            for i in (self.current_page + 1)..=self.total_pages {
                v.push(PageItem::Page(unsafe { NonZeroUsize::new_unchecked(i) }));
            }
        }

        if show_next {
            v.push(PageItem::Next(unsafe { NonZeroUsize::new_unchecked(self.current_page + 1) }));
        }

        v
    }
}
