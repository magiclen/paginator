#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum YesNoDepends {
    Yes,
    No,
    Depends,
}

impl YesNoDepends {
    #[inline]
    pub fn yes(self) -> bool {
        match self {
            YesNoDepends::Yes => true,
            _ => false,
        }
    }

    #[inline]
    pub fn depends(self) -> bool {
        match self {
            YesNoDepends::Depends => true,
            _ => false,
        }
    }

    #[inline]
    pub fn no(self) -> bool {
        match self {
            YesNoDepends::No => true,
            _ => false,
        }
    }
}
