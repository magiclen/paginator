use core::{
    fmt::{self, Display, Formatter},
    num::NonZeroUsize,
};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PageItem {
    Page(NonZeroUsize),
    CurrentPage(NonZeroUsize),
    Ignore,
    Prev(NonZeroUsize),
    Next(NonZeroUsize),
    ReservedPrev,
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
