#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum YesNoDepends {
    Yes,
    No,
    Depends,
}

impl YesNoDepends {
    #[inline]
    pub fn yes(self) -> bool {
        matches!(self, YesNoDepends::Yes)
    }

    #[inline]
    pub fn depends(self) -> bool {
        matches!(self, YesNoDepends::Depends)
    }

    #[inline]
    pub fn no(self) -> bool {
        matches!(self, YesNoDepends::No)
    }
}
