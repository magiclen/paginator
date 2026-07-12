/// A setting that can be enabled, disabled, or decided from the page state.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum YesNoDepends {
    /// Always enable the item.
    Yes,
    /// Always disable the item.
    No,
    /// Enable the item only when the page state requires it.
    Depends,
}

impl YesNoDepends {
    #[inline]
    /// Return whether this value is `Yes`.
    pub fn yes(self) -> bool {
        matches!(self, YesNoDepends::Yes)
    }

    #[inline]
    /// Return whether this value is `Depends`.
    pub fn depends(self) -> bool {
        matches!(self, YesNoDepends::Depends)
    }

    #[inline]
    /// Return whether this value is `No`.
    pub fn no(self) -> bool {
        matches!(self, YesNoDepends::No)
    }
}
