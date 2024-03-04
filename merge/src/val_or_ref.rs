use std::{borrow::Borrow, fmt, hash::Hash, ops::Deref};

pub enum ValOrRef<'a, T: ?Sized + ToOwned + 'a> {
    /// A static reference to a value.
    Borrowed(&'a T),
    /// An owned value.
    Owned(<T as ToOwned>::Owned),
}

impl<'a, T: ?Sized + ToOwned> ValOrRef<'a, T> {
    /// Converts the value into an owned value.
    pub fn into_owned(self) -> <T as ToOwned>::Owned {
        match self {
            ValOrRef::Borrowed(v) => v.to_owned(),
            ValOrRef::Owned(v) => v,
        }
    }

    pub const fn is_borrowed(&self) -> bool {
        matches!(self, ValOrRef::Borrowed(_))
    }

    pub const fn is_owned(&self) -> bool {
        matches!(self, ValOrRef::Owned(_))
    }
}

impl<T: ?Sized + ToOwned> Deref for ValOrRef<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        match self {
            ValOrRef::Borrowed(v) => v,
            ValOrRef::Owned(v) => v.borrow(),
        }
    }
}

impl<T: ?Sized + ToOwned> Borrow<T> for ValOrRef<'_, T> {
    #[inline(always)]
    fn borrow(&self) -> &T {
        self.deref()
    }
}

impl<T: ?Sized + ToOwned> AsRef<T> for ValOrRef<'_, T> {
    #[inline(always)]
    fn as_ref(&self) -> &T {
        self.deref()
    }
}

impl<T: ?Sized + Hash> Hash for ValOrRef<'_, T>
where
    T: ToOwned,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (**self).hash(state)
    }
}

impl<T: ?Sized + fmt::Debug> fmt::Debug for ValOrRef<'_, T>
where
    T: ToOwned,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (**self).fmt(f)
    }
}

impl From<String> for ValOrRef<'_, str> {
    fn from(v: String) -> Self {
        ValOrRef::Owned(v)
    }
}

impl<'a, T> Clone for ValOrRef<'a, T>
where
    T: ?Sized + ToOwned + 'a,
    <T as ToOwned>::Owned: Clone,
{
    fn clone(&self) -> Self {
        match self {
            Self::Borrowed(v) => Self::Borrowed(*v),
            Self::Owned(v) => Self::Owned(v.clone()),
        }
    }
}

impl From<ValOrRef<'_, str>> for String {
    fn from(v: ValOrRef<'_, str>) -> Self {
        match v {
            ValOrRef::Borrowed(v) => v.to_owned(),
            ValOrRef::Owned(v) => v,
        }
    }
}
impl<'a, 'b, A: ?Sized, B: ?Sized> PartialEq<ValOrRef<'b, B>> for ValOrRef<'a, A>
where
    A: PartialEq<B>,
    A: ToOwned,
    B: ToOwned,
{
    fn eq(&self, other: &ValOrRef<'b, B>) -> bool {
        **self == **other
    }
}

impl<T: ?Sized + ToOwned + Eq> Eq for ValOrRef<'_, T> {}

impl<'a, 'b, A: ?Sized, B: ?Sized> PartialOrd<ValOrRef<'b, B>> for ValOrRef<'a, A>
where
    A: PartialOrd<B>,
    A: ToOwned,
    B: ToOwned,
{
    fn partial_cmp(&self, other: &ValOrRef<'b, B>) -> Option<std::cmp::Ordering> {
        (**self).partial_cmp(&**other)
    }
}
