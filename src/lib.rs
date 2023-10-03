//! A simple crate to join the elements of iterators, interspercing a separator between all elements.
//! 
//! This is done somewhat efficiently, if possible, meaning if the iterator is cheaply clonable you can
//! directly print the result of [`Join::join()`] without creating a temporary [`String`] in memory.
//!
//! ```
//! use join_string::Join;
//! 
//! assert_eq!(
//!     "foo bar baz".split_whitespace().join(", ").into_string(),
//!     "foo, bar, baz");
//! 
//! println!("{}",
//!     "foo bar baz".split_whitespace()
//!         .map(|s| s.chars().rev().join(""))
//!         .join(' '));
//! // Output: oof rab zab
//! ```

/// Trait that provides a method to join elements of an iterator, interspercing a separator between all elements.
pub trait Join<I> where I: std::iter::Iterator, I::Item: std::fmt::Display {
    /// Joins elements of an iterator, interspercing the given separator between all elements.
    /// 
    /// The return value is a [`Joiner`] that hasn't done the joining yet. It either can be
    /// used wherever a [`std::fmt::Display`] is expected (e.g. when formatting), or you can convert
    /// it into a [`String`] using the [`Joiner::into_string()`] method.
    fn join<S>(self, sep: S) -> Joiner<I, S> where S: std::fmt::Display;
}

#[derive(Debug)]
pub struct Joiner<I, S> where I: std::iter::Iterator, S: std::fmt::Display {
    iter: I,
    sep: S
}

impl<I, S> Joiner<I, S> where I: std::iter::Iterator, S: std::fmt::Display, I::Item: std::fmt::Display {
    /// Consumes the backing iterator of a [`Joiner`] and returns the joined elements as a new [`String`].
    #[inline]
    pub fn into_string(self) -> String {
        let mut buffer = String::new();
        let _ = self.write_fmt(&mut buffer);
        return buffer;
    }

    /// Consumes the backing iterator of a [`Joiner`] and writes the joined elements into a [`std::fmt::Write`].
    pub fn write_fmt<W: std::fmt::Write>(mut self, mut writer: W) -> std::fmt::Result {
        if let Some(first) = self.iter.next() {
            write!(writer, "{}", first)?;
            while let Some(item) = self.iter.next() {
                write!(writer, "{}{}", self.sep, item)?;
            }
        }
        Ok(())
    }

    /// Consumes the backing iterator of a [`Joiner`] and writes the joined elements into a [`std::io::Write`].
    pub fn write_io<W: std::io::Write>(mut self, mut writer: W) -> std::io::Result<()> {
        if let Some(first) = self.iter.next() {
            write!(writer, "{}", first)?;
            while let Some(item) = self.iter.next() {
                write!(writer, "{}{}", self.sep, item)?;
            }
        }
        Ok(())
    }
}

impl<I, S> Into<String> for Joiner<I, S> where I: std::iter::Iterator, S: std::fmt::Display, I::Item: std::fmt::Display {
    #[inline]
    fn into(self) -> String {
        self.into_string()
    }
}

impl<I, S> Clone for Joiner<I, S> where I: std::iter::Iterator, S: std::fmt::Display, I::Item: std::fmt::Display, I: Clone, S: Clone {
    #[inline]
    fn clone(&self) -> Self {
        Joiner {
            iter: self.iter.clone(),
            sep: self.sep.clone()
        }
    }
}

impl<I> Join<I> for I where I: std::iter::Iterator, I::Item: std::fmt::Display {
    #[inline]
    fn join<S>(self, sep: S) -> Joiner<I, S> where S: std::fmt::Display {
        Joiner {
            iter: self,
            sep
        }
    }
}

pub trait Iterable<I: std::iter::Iterator> {
    fn iter(self) -> I;
}

impl<I> Iterable<I> for I where I: std::iter::Iterator {
    #[inline]
    fn iter(self) -> I {
        self
    }
}

impl<'a, T> Iterable<core::slice::Iter<'a, T>> for &'a [T] {
    #[inline]
    fn iter(self) -> core::slice::Iter<'a, T> {
        self.iter()
    }
}

impl<'a, T, const N: usize> Iterable<core::slice::Iter<'a, T>> for &'a [T; N] {
    #[inline]
    fn iter(self) -> core::slice::Iter<'a, T> {
        self.as_slice().iter()
    }
}

impl<'a, T> Iterable<core::slice::Iter<'a, T>> for &'a Vec<T> {
    #[inline]
    fn iter(self) -> core::slice::Iter<'a, T> {
        self.as_slice().iter()
    }
}

/// Join anything that implements [`Iterable`], not just iterators.
/// The elements need to implement [`std::fmt::Display`].
/// 
/// You can pass iterators, slices, and borrows of arrays and [`Vec`]s:
/// 
/// ```
/// use join_string::join;
/// 
/// assert_eq!(
///     join(&["foo", "bar", "baz"], ", ").into_string(),
///     "foo, bar, baz"
/// );
/// 
/// assert_eq!(
///     join([1, 2, 3].as_slice(), ", ").into_string(),
///     "1, 2, 3"
/// );
/// 
/// assert_eq!(
///     join(&vec!['a', 'b', 'c'], ", ").into_string(),
///     "a, b, c"
/// );
/// 
/// assert_eq!(
///     join([
///         "foo".to_owned(),
///         "bar".to_owned(),
///         "baz".to_owned()
///     ].iter().rev(), ", ").into_string(),
///     "baz, bar, foo"
/// );
/// ```
#[inline]
pub fn join<I, S>(elements: impl Iterable<I>, sep: S) -> Joiner<I, S> where I: std::iter::Iterator, I::Item: std::fmt::Display, S: std::fmt::Display {
    elements.iter().join(sep)
}

/// Helper for joining elements that only implement [`AsRef<str>`], but not [`std::fmt::Display`].
#[repr(transparent)]
#[derive(Debug)]
pub struct DisplayWrapper<T: AsRef<str>> ( T );

impl<T: AsRef<str>> DisplayWrapper<T> {
    #[inline]
    pub fn new(value: T) -> Self {
        Self (value)
    }

    /// Map an iterable of [`AsRef<str>`] elements to an iterator of [`std::fmt::Display`] elements.
    #[inline]
    pub fn map<I>(elements: impl Iterable<I>) -> impl std::iter::Iterator<Item = impl std::fmt::Display>
    where I: std::iter::Iterator<Item = T> {
        elements.iter().map(|item| Self (item))
    }
}

impl<T> std::fmt::Display for DisplayWrapper<T> where T: AsRef<str> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.as_ref().fmt(f)
    }
}

impl<T> Clone for DisplayWrapper<T> where T: AsRef<str>, T: Clone {
    #[inline]
    fn clone(&self) -> Self {
        Self (self.0.clone())
    }
}

/// Join anything that implements [`Iterable`], not just iterators when elements
/// don't implement [`std::fmt::Display`], but implement [`AsRef<str>`] instead.
/// 
/// You can pass iterators, slices, and borrows of arrays and [`Vec`]s:
/// 
/// ```
/// use join_string::join_str;
/// 
/// assert_eq!(
///     join_str(&["foo", "bar", "baz"], ", ").into_string(),
///     "foo, bar, baz"
/// );
/// 
/// assert_eq!(
///     join_str([
///         &"foo".to_owned(),
///         &"bar".to_owned(),
///         &"baz".to_owned()
///     ].as_slice(), ", ").into_string(),
///     "foo, bar, baz"
/// );
/// 
/// assert_eq!(
///     join_str(&vec!["foo", "bar", "baz"], ", ").into_string(),
///     "foo, bar, baz"
/// );
/// 
/// assert_eq!(
///     join_str([
///         "foo".to_owned(),
///         "bar".to_owned(),
///         "baz".to_owned()
///     ].iter().rev(), ", ").into_string(),
///     "baz, bar, foo"
/// );
/// ```
#[inline]
pub fn join_str<I, S>(elements: impl Iterable<I>, sep: S) -> Joiner<impl std::iter::Iterator<Item = impl std::fmt::Display>, impl std::fmt::Display>
where I: std::iter::Iterator, I::Item: AsRef<str>, S: AsRef<str> {
    DisplayWrapper::map(elements).join(DisplayWrapper (sep))
}

impl<'a, T> Join<core::slice::Iter<'a, T>> for &'a [T] where T: std::fmt::Display {
    #[inline]
    fn join<S>(self, sep: S) -> Joiner<core::slice::Iter::<'a, T>, S> where S: std::fmt::Display {
        self.iter().join(sep)
    }
}

impl<I, S> std::fmt::Display for Joiner<I, S> where I: std::iter::Iterator, S: std::fmt::Display, I::Item: std::fmt::Display, I: Clone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.iter.clone();
        if let Some(first) = iter.next() {
            first.fmt(f)?;
            while let Some(item) = iter.next() {
                self.sep.fmt(f)?;
                item.fmt(f)?;
            }
        }
        Ok(())
    }
}
