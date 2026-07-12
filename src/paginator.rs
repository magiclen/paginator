use alloc::vec::Vec;
use core::num::NonZeroUsize;

use crate::{PageItem, YesNoDepends};

macro_rules! non_zero_page {
    ($page:expr) => {{
        let page = $page;
        debug_assert!(page > 0);

        // SAFETY: Every caller passes a page number that is greater than zero.
        unsafe { NonZeroUsize::new_unchecked(page) }
    }};
}

/// Pagination settings for one current page.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Paginator {
    // total_pages and current_page is always bigger than 0 and current_page CANNOT be bigger than total_pages.
    pub(crate) total_pages:    usize,
    pub(crate) current_page:   usize,
    pub(crate) max_item_count: usize,
    pub(crate) start_size:     usize,
    pub(crate) end_size:       usize,
    pub(crate) has_prev:       YesNoDepends,
    pub(crate) has_next:       YesNoDepends,
}

impl Paginator {
    /// Return the total number of pages.
    #[inline]
    pub const fn total_pages(&self) -> usize {
        self.total_pages
    }

    /// Return the current page number.
    #[inline]
    pub const fn current_page(&self) -> usize {
        self.current_page
    }

    /// Return the maximum number of generated items.
    #[inline]
    pub const fn max_item_count(&self) -> usize {
        self.max_item_count
    }

    /// Return the number of pages reserved at the start edge.
    #[inline]
    pub const fn start_size(&self) -> usize {
        self.start_size
    }

    /// Return the number of pages reserved at the end edge.
    #[inline]
    pub const fn end_size(&self) -> usize {
        self.end_size
    }

    /// Return the setting for the previous-page item.
    #[inline]
    pub const fn has_prev(&self) -> YesNoDepends {
        self.has_prev
    }

    /// Return the setting for the next-page item.
    #[inline]
    pub const fn has_next(&self) -> YesNoDepends {
        self.has_next
    }
}

impl Paginator {
    /// Create `PageItem`s.
    pub fn paginate(&self) -> Vec<PageItem> {
        // Reserve space for page items and up to two control items without overflowing usize.
        let page_capacity = self.max_item_count.min(self.total_pages);
        let control_capacity = (self.max_item_count - page_capacity).min(2);
        let mut v = Vec::with_capacity(page_capacity + control_capacity);
        let mut items_counter = self.max_item_count;

        let show_prev = self.has_prev.yes()
            || (self.has_prev.depends() && self.current_page > 1 && self.total_pages > 2);
        let show_next = self.has_next.yes()
            || (self.has_next.depends()
                && self.current_page < self.total_pages
                && self.total_pages > 2);

        if show_prev {
            let page = self.current_page - 1;

            if page == 0 {
                v.push(PageItem::ReservedPrev);
            } else {
                v.push(PageItem::Prev(non_zero_page!(page)));
            }

            items_counter -= 1;
        }

        if show_next {
            items_counter -= 1;
        }

        let start_size = self.start_size.min(self.total_pages);
        let end_size = self.end_size.min(self.total_pages);

        let (ignore_start, ignore_end) = if self.total_pages > items_counter {
            let ignore_start = self.current_page > start_size && self.current_page - start_size > 2;
            let ignore_end = self.total_pages - self.current_page > end_size;

            (ignore_start, ignore_end)
        } else {
            (false, false)
        };

        if ignore_start {
            items_counter -= start_size + 2; // start_size + 1 + 1

            if ignore_end {
                items_counter -= end_size + 1;

                for i in 1..=start_size {
                    v.push(PageItem::Page(non_zero_page!(i)));
                }

                let window_size = items_counter >> 1;

                // Keep the missing left distance so the window can shift to the right by the same amount.
                let (mut hp_s, hp_s_underflow) = match self.current_page.checked_sub(window_size) {
                    Some(hp_s) => (hp_s, 0),
                    None => (0, window_size - self.current_page),
                };
                let hp_e = self.current_page - 1;
                let tp_s = self.current_page + 1;
                let right_window_size = items_counter - window_size;
                let max_right_window_size = usize::MAX - self.current_page;
                // Keep the overflow distance so the window can shift to the left by the same amount.
                let (mut tp_e, tp_e_overflow) = if right_window_size <= max_right_window_size {
                    (self.current_page + right_window_size, 0)
                } else {
                    (usize::MAX, right_window_size - max_right_window_size)
                };

                let end_boundary = self.total_pages - end_size;

                if hp_s_underflow > 0 || start_size + 2 >= hp_s {
                    // Reuse the ignore marker slot when the left gap is too small to hide.

                    let old_hp_s = hp_s;

                    hp_s = start_size + 1;

                    let shift = hp_s + 1 - old_hp_s + hp_s_underflow;

                    tp_e += shift;
                } else {
                    v.push(PageItem::Ignore);

                    if tp_e >= end_boundary {
                        // Shift the visible window left when it reaches the reserved end section.
                        let old_tp_e = tp_e;

                        tp_e = end_boundary - 1;
                        hp_s -= old_tp_e - tp_e + tp_e_overflow;
                    }
                }

                for i in hp_s..=hp_e {
                    v.push(PageItem::Page(non_zero_page!(i)));
                }

                v.push(PageItem::CurrentPage(non_zero_page!(self.current_page)));

                for i in tp_s..=tp_e {
                    v.push(PageItem::Page(non_zero_page!(i)));
                }

                if tp_e == end_boundary - 1 {
                    v.push(PageItem::Page(non_zero_page!(end_boundary)));
                } else {
                    v.push(PageItem::Ignore);
                }

                if end_size > 0 {
                    for i in (end_boundary + 1)..=self.total_pages {
                        v.push(PageItem::Page(non_zero_page!(i)));
                    }
                }
            } else {
                if self.current_page < self.total_pages {
                    items_counter -= self.total_pages - self.current_page;
                }

                for i in 1..=start_size {
                    v.push(PageItem::Page(non_zero_page!(i)));
                }

                v.push(PageItem::Ignore);

                for i in (self.current_page - items_counter)..=(self.current_page - 1) {
                    v.push(PageItem::Page(non_zero_page!(i)));
                }

                v.push(PageItem::CurrentPage(non_zero_page!(self.current_page)));

                if self.current_page < self.total_pages {
                    for i in (self.current_page + 1)..=self.total_pages {
                        v.push(PageItem::Page(non_zero_page!(i)));
                    }
                }
            }
        } else if ignore_end {
            items_counter -= end_size + 1;

            items_counter -= self.current_page;

            for i in 1..self.current_page {
                v.push(PageItem::Page(non_zero_page!(i)));
            }

            v.push(PageItem::CurrentPage(non_zero_page!(self.current_page)));

            let window_end = self.current_page + items_counter;

            for i in (self.current_page + 1)..=window_end {
                v.push(PageItem::Page(non_zero_page!(i)));
            }

            v.push(PageItem::Ignore);

            if end_size > 0 {
                for i in (self.total_pages - end_size + 1)..=self.total_pages {
                    v.push(PageItem::Page(non_zero_page!(i)));
                }
            }
        } else {
            for i in 1..self.current_page {
                v.push(PageItem::Page(non_zero_page!(i)));
            }

            v.push(PageItem::CurrentPage(non_zero_page!(self.current_page)));

            if self.current_page < self.total_pages {
                for i in (self.current_page + 1)..=self.total_pages {
                    v.push(PageItem::Page(non_zero_page!(i)));
                }
            }
        }

        if show_next {
            if self.current_page < self.total_pages {
                v.push(PageItem::Next(non_zero_page!(self.current_page + 1)));
            } else {
                v.push(PageItem::ReservedNext);
            }
        }

        v
    }
}
