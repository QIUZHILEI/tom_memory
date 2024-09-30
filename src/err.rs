use core::{
    alloc::{Layout, LayoutError},
    error::Error,
    fmt::Display,
};
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AllocError {
    Layout(LayoutError),
    Misaligned(usize),
    OutOfMemory(Layout),
    NullPointer(usize),
}

impl Error for AllocError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self)
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }

    fn provide<'a>(&'a self, request: &mut core::error::Request<'a>) {
        request.provide_value(self.clone());
    }
}

impl From<LayoutError> for AllocError {
    fn from(value: LayoutError) -> Self {
        Self::Layout(value)
    }
}

impl Display for AllocError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            AllocError::Layout(err) => write!(f, "{}", err),
            AllocError::OutOfMemory(layout) => {
                write!(f, "The size of {} is out of memory!", layout.size())
            }
            AllocError::NullPointer(addr) => write!(f, "The address of {} is null!", *addr),
            AllocError::Misaligned(size) => write!(f, "The size of {} is misaligned!", *size),
        }
    }
}
