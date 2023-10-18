//! A simple crate to join elements as a string, interspersing a separator between
//! all elements.
//!
//! This is done somewhat efficiently, if possible. Meaning if the iterator is
//! cheaply clonable you can directly print the result of [`Join::join()`]
//! without creating a temporary [`String`] in memory. The [`Join::join()`]
//! method will appear on anything that implements [`std::iter::IntoIterator`],
//! meaning on all iterators and collections. The elements and the separator
//! need to implement [`std::fmt::Display`]. Alternatively the
//! [`Join::join_str()`] method can be used to join elements that only
//! implement [`AsRef<str>`].
//!
//! # Examples
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
//!
//! You can also write the result more directly to a [`std::io::Write`] or
//! [`std::fmt::Write`] even if the backing iterator doesn't implement
//! [`Clone`](https://doc.rust-lang.org/std/clone/trait.Clone.html).
//!
//! ```
//! # use join_string::Join;
//! #
//! # fn main() -> std::io::Result<()> {
//! ["foo", "bar", "baz"].join(", ").write_io(std::io::stdout())?;
//! # Ok(())
//! # }
//! ```
//!
//! ```
//! # use join_string::Join;
//! #
//! # fn main() -> std::fmt::Result {
//! let mut str = String::new();
//! ["foo", "bar", "baz"].join(", ").write_fmt(&mut str)?;
//! # Ok(())
//! # }
//! ```
//!
//! # Notes
//!
//! The standard library already provides a similar [`std::slice::Join`]
//! trait on slices, but not on iterators, and the standard library version
//! always directly returns a new [`String`]. Further there are multiple
//! other similar crates that however work a bit differently, e.g. having
//! more restrictions on element and separator types or always returning a
//! [`String`].

// =============================================================================
//      struct Joiner
// =============================================================================

/// Helper struct that captures the iterator and separator for later joining.
pub struct Joiner<I, S>
where
    I: std::iter::Iterator,
    S: std::fmt::Display,
{
    iter: I,
    sep: S,
}

impl<I, S> Joiner<I, S>
where
    I: std::iter::Iterator,
    S: std::fmt::Display,
{
    /// Create a [`Joiner`] object.
    ///
    /// You can use this when implementing your own `join()` function.
    #[inline]
    pub fn new(iter: I, sep: S) -> Self {
        Self { iter, sep }
    }

    /// Consumes the backing iterator of a [`Joiner`] and returns the joined elements as a new [`String`].
    #[inline]
    pub fn into_string(self) -> String
    where I::Item: std::fmt::Display {
        let mut buffer = String::new();
        let _ = self.write_fmt(&mut buffer);
        buffer
    }

    /// Consumes the backing iterator of a [`Joiner`] and writes the joined elements into a [`std::fmt::Write`].
    pub fn write_fmt<W: std::fmt::Write>(mut self, mut writer: W) -> std::fmt::Result
    where I::Item: std::fmt::Display {
        if let Some(first) = self.iter.next() {
            write!(writer, "{}", first)?;
            for item in self.iter {
                write!(writer, "{}{}", self.sep, item)?;
            }
        }
        Ok(())
    }

    /// Consumes the backing iterator of a [`Joiner`] and writes the joined elements into a [`std::io::Write`].
    pub fn write_io<W: std::io::Write>(mut self, mut writer: W) -> std::io::Result<()>
    where I::Item: std::fmt::Display {
        if let Some(first) = self.iter.next() {
            write!(writer, "{}", first)?;
            for item in self.iter {
                write!(writer, "{}{}", self.sep, item)?;
            }
        }
        Ok(())
    }
}

impl<I, S> From<Joiner<I, S>> for String
where
    I: std::iter::Iterator,
    S: std::fmt::Display,
    I::Item: std::fmt::Display,
{
    #[inline]
    fn from(value: Joiner<I, S>) -> Self {
        value.into_string()
    }
}

impl<I, S> Clone for Joiner<I, S>
where
    I: std::iter::Iterator,
    S: std::fmt::Display,
    I::Item: std::fmt::Display,
    I: Clone,
    S: Clone,
{
    #[inline]
    fn clone(&self) -> Self {
        Self {
            iter: self.iter.clone(),
            sep: self.sep.clone(),
        }
    }
}

impl<I, S> std::fmt::Display for Joiner<I, S>
where
    I: std::iter::Iterator,
    S: std::fmt::Display,
    I::Item: std::fmt::Display,
    I: Clone,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.iter.clone();
        if let Some(first) = iter.next() {
            first.fmt(f)?;
            for item in iter {
                self.sep.fmt(f)?;
                item.fmt(f)?;
            }
        }
        Ok(())
    }
}

impl<I, S> std::fmt::Debug for Joiner<I, S>
where
    I: std::iter::Iterator,
    S: std::fmt::Display,
    I::Item: std::fmt::Debug,
    I: Clone,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.iter.clone();
        if let Some(first) = iter.next() {
            first.fmt(f)?;
            for item in iter {
                self.sep.fmt(f)?;
                item.fmt(f)?;
            }
        }
        Ok(())
    }
}
// =============================================================================
//      trait Join
// =============================================================================

/// Trait that provides a method to join elements of an iterator, interspersing
/// a separator between all elements.
///
/// This trait is implemented for anything that implements
/// [`std::iter::IntoIterator`], which is e.g. arrays, slices, [`Vec`], and more.
pub trait Join<I: std::iter::Iterator>: std::iter::IntoIterator<IntoIter = I> {
    /// Join the elements of an iterator, interspersing a separator between
    /// all elements.
    ///
    /// The elements and the separator need to implement [`std::fmt::Display`].
    #[inline]
    fn join<S>(self, sep: S) -> Joiner<I, S>
    where
        Self: Sized,
        S: std::fmt::Display,
    {
        Joiner {
            iter: self.into_iter(),
            sep,
        }
    }

    /// Join the elements of an iterator, interspersing a separator between
    /// all elements.
    ///
    /// The elements and the separator need to implement [`AsRef<str>`].
    #[inline]
    fn join_str<S>(self, sep: S) -> Joiner<DisplayIter<I>, DisplayWrapper<S>>
    where
        Self: Sized,
        S: AsRef<str>,
        I::Item: AsRef<str>,
    {
        Joiner {
            iter: DisplayIter {
                iter: self.into_iter(),
            },
            sep: DisplayWrapper(sep),
        }
    }
}

impl<T> Join<T::IntoIter> for T where T: std::iter::IntoIterator {}

// =============================================================================
//      struct DisplayWrapper
// =============================================================================

/// Helper for joining elements that only implement [`AsRef<str>`], but not [`std::fmt::Display`].
#[repr(transparent)]
#[derive(Debug)]
pub struct DisplayWrapper<T: AsRef<str>>(T);

impl<T: AsRef<str>> DisplayWrapper<T> {
    #[inline]
    pub fn new(value: T) -> Self {
        Self(value)
    }
}

impl<T> std::fmt::Display for DisplayWrapper<T>
where
    T: AsRef<str>,
{
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.as_ref().fmt(f)
    }
}

impl<T> Clone for DisplayWrapper<T>
where
    T: AsRef<str>,
    T: Clone,
{
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> AsRef<str> for DisplayWrapper<T>
where
    T: AsRef<str>,
{
    #[inline]
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

// =============================================================================
//      struct DisplayIter
// =============================================================================

/// Iterator-facade that maps an iterator over [`AsRef<str>`] to an iterator
/// over [`DisplayWrapper`].
///
/// This is used to implement [`Join::join_str()`].
#[derive(Debug)]
pub struct DisplayIter<I>
where
    I: std::iter::Iterator,
{
    iter: I,
}

impl<I> DisplayIter<I>
where
    I: std::iter::Iterator,
{
    #[inline]
    pub fn new(elements: impl Join<I>) -> Self {
        Self {
            iter: elements.into_iter(),
        }
    }
}

impl<I> std::iter::Iterator for DisplayIter<I>
where
    I: std::iter::Iterator,
    I::Item: AsRef<str>,
{
    type Item = DisplayWrapper<I::Item>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.iter.next() {
            return Some(DisplayWrapper(item));
        }
        None
    }

    #[inline]
    fn last(self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        if let Some(item) = self.iter.last() {
            return Some(DisplayWrapper(item));
        }
        None
    }

    #[inline]
    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.iter.count()
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.iter.nth(n).map(DisplayWrapper)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    #[inline]
    #[cfg(target_feature = "iter_advance_by")]
    fn advance_by(&mut self, n: usize) -> Result<(), NonZeroUsize> {
        self.iter.advance_by(n)
    }

    #[inline]
    #[cfg(target_feature = "trusted_random_access")]
    unsafe fn __iterator_get_unchecked(&mut self, idx: usize) -> Self::Item
    where
        Self: TrustedRandomAccessNoCoerce,
    {
        DisplayWrapper(self.iter.__iterator_get_unchecked(idx))
    }
}

impl<I> std::iter::ExactSizeIterator for DisplayIter<I>
where
    I: std::iter::ExactSizeIterator,
    I::Item: AsRef<str>,
{
    #[inline]
    fn len(&self) -> usize {
        self.iter.len()
    }

    #[inline]
    #[cfg(target_feature = "exact_size_is_empty")]
    fn is_empty(&self) -> bool {
        self.iter.is_empty()
    }
}

impl<I> std::iter::DoubleEndedIterator for DisplayIter<I>
where
    I: std::iter::DoubleEndedIterator,
    I::Item: AsRef<str>,
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.iter.next_back() {
            return Some(DisplayWrapper(item));
        }
        None
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        if let Some(item) = self.iter.nth_back(n) {
            return Some(DisplayWrapper(item));
        }
        None
    }

    #[inline]
    #[cfg(target_feature = "iter_advance_by")]
    fn advance_back_by(&mut self, n: usize) -> Result<(), NonZeroUsize> {
        self.iter.advance_back_by(n)
    }
}

impl<I> Clone for DisplayIter<I>
where
    I: std::iter::Iterator,
    I: Clone,
{
    #[inline]
    fn clone(&self) -> Self {
        Self {
            iter: self.iter.clone(),
        }
    }
}

// =============================================================================
//      functions
// =============================================================================

/// Join anything that implements [`Join`]. The elements need to implement
/// [`std::fmt::Display`].
///
/// # Examples
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
pub fn join<I, S>(elements: impl Join<I>, sep: S) -> Joiner<I, S>
where
    I: std::iter::Iterator,
    S: std::fmt::Display,
{
    elements.join(sep)
}

/// Join anything that implements [`Join`] when elements don't implement
/// [`std::fmt::Display`], but implement [`AsRef<str>`] instead.
///
/// # Examples
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
pub fn join_str<I, S>(
    elements: impl Join<I>,
    sep: S,
) -> Joiner<impl std::iter::Iterator<Item = impl std::fmt::Display>, impl std::fmt::Display>
where
    I: std::iter::Iterator,
    I::Item: AsRef<str>,
    S: AsRef<str>,
{
    DisplayIter::new(elements).join(DisplayWrapper(sep))
}
