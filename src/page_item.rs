use core::{
    fmt::{self, Display, Formatter},
    num::NonZeroUsize,
};

/// An item displayed in a pagination bar.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PageItem {
    /// A link to a regular page.
    Page(NonZeroUsize),
    /// The current page without a regular link.
    CurrentPage(NonZeroUsize),
    /// A marker for a hidden range of pages.
    Ignore,
    /// A link to the previous page.
    Prev(NonZeroUsize),
    /// A link to the next page.
    Next(NonZeroUsize),
    /// A reserved previous-page position without a valid target.
    ReservedPrev,
    /// A reserved next-page position without a valid target.
    ReservedNext,
}

impl Display for PageItem {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            PageItem::Page(page) => f.write_fmt(format_args!("{}", page)),
            PageItem::CurrentPage(page) => f.write_fmt(format_args!("{}*", page)),
            PageItem::Ignore => f.write_str("..."),
            PageItem::Next(_) => f.write_str(">"),
            PageItem::Prev(_) => f.write_str("<"),
            PageItem::ReservedPrev => f.write_str(")"),
            PageItem::ReservedNext => f.write_str("("),
        }
    }
}
