use std::fmt::{Debug, Display, Formatter};

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Version {
    major: u32,
    minor: u32,
    patch: u32,
    build: u32,
}

impl Debug for Version {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}.{}", self.major, self.minor, self.patch, self.build)
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}