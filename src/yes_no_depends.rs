/// A setting that can be enabled, disabled, or decided from the page state.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum YesNoDepends {
    /// Always enable the item, using `PageItem::ReservedPrev` or `PageItem::ReservedNext` when there is no target page.
    Yes,
    /// Always disable the item.
    No,
    /// Enable the item only when the target page exists and `total_pages` is greater than 2.
    Depends,
}

impl YesNoDepends {
    /// Return whether this value is `Yes`.
    #[inline]
    pub const fn yes(self) -> bool {
        matches!(self, YesNoDepends::Yes)
    }

    /// Return whether this value is `Depends`.
    #[inline]
    pub const fn depends(self) -> bool {
        matches!(self, YesNoDepends::Depends)
    }

    /// Return whether this value is `No`.
    #[inline]
    pub const fn no(self) -> bool {
        matches!(self, YesNoDepends::No)
    }
}
