//! Provider of [`Staff`].

use std::ops::{Deref, DerefMut};

/// A pointer type that owns or borrows data.
pub enum Staff<'a, T: ?Sized + 'a> {
    /// Target data is owned.
    Own(Box<T>),
    /// Target data is borrowed.
    Borrow(&'a mut T),
}

impl<'a, T: ?Sized> Staff<'a, T> {
    /// Create owned instance.
    #[must_use]
    pub fn new_own(x: Box<T>) -> Self {
        Self::Own(x)
    }

    /// Create borrowed instance.
    #[must_use]
    pub fn new_borrow(x: &'a mut T) -> Self {
        Self::Borrow(x)
    }
}

impl<T: ?Sized> Deref for Staff<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Own(x) => x.deref(),
            Self::Borrow(x) => x,
        }
    }
}

impl<T: ?Sized> DerefMut for Staff<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::Own(x) => x.deref_mut(),
            Self::Borrow(x) => x,
        }
    }
}
