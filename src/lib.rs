//! A simple crate to join the elements of iterators, interspercing a separator between all elements.
//! 
//! This is done somewhat efficiently, if possible, meaning if the iterator is cheaply clonable you can
//! directly print the result of [`StringJoin::join()`] without creating a temporary [`String`] in memory.
//!
//! ```
//! use join_string::StringJoin;
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
/// 
pub trait StringJoin<I, S> where I: std::iter::Iterator, S: std::fmt::Display, I::Item: std::fmt::Display {
    /// Joins elements of an iterator, interspercing the given separator between all elements.
    /// 
    /// The return value is a [`StringJoiner`] that hasn't done the joining yet. It either can be
    /// used wherever a [`std::fmt::Display`] is expected (e.g. when formatting), or you can convert
    /// it into a [`String`] using the [`StringJoiner::into_string()`] method.
    fn join(self, sep: S) -> StringJoiner<I, S>;
}

#[derive(Debug)]
pub struct StringJoiner<I, S> where I: std::iter::Iterator, S: std::fmt::Display {
    iter: I,
    sep: S
}

impl<I, S> StringJoiner<I, S> where I: std::iter::Iterator, S: std::fmt::Display, I::Item: std::fmt::Display {
    /// Consumes the backing iterator of a [`StringJoiner`] and returns the joined elements as a new [`String`].
    #[inline]
    pub fn into_string(self) -> String {
        let mut buffer = String::new();
        self.write_join(&mut buffer);
        return buffer;
    }

    /// Consumes the backing iterator of a [`StringJoiner`] and writes the joined elements into a [`std::fmt::Write`].
    pub fn write_join<W: std::fmt::Write>(mut self, writer: &mut W) {
        if let Some(first) = self.iter.next() {
            let _ = write!(writer, "{}", first);
            while let Some(item) = self.iter.next() {
                let _ = write!(writer, "{}{}", self.sep, item);
            }
        }
    }
}

impl<I, S> Into<String> for StringJoiner<I, S> where I: std::iter::Iterator, S: std::fmt::Display, I::Item: std::fmt::Display {
    #[inline]
    fn into(self) -> String {
        self.into_string()
    }
}

impl<I, S> Clone for StringJoiner<I, S> where I: std::iter::Iterator, S: std::fmt::Display, I::Item: std::fmt::Display, I: Clone, S: Clone {
    #[inline]
    fn clone(&self) -> Self {
        StringJoiner {
            iter: self.iter.clone(),
            sep: self.sep.clone()
        }
    }
}

impl<I, S> StringJoin<I, S> for I where I: std::iter::Iterator, I::Item: std::fmt::Display, S: std::fmt::Display {
    #[inline]
    fn join(self, sep: S) -> StringJoiner<I, S> {
        StringJoiner {
            iter: self,
            sep
        }
    }
}

impl<'a, T, S> StringJoin<core::slice::Iter<'a, T>, S> for &'a [T] where T: std::fmt::Display, S: std::fmt::Display {
    #[inline]
    fn join(self, sep: S) -> StringJoiner<core::slice::Iter::<'a, T>, S> {
        self.iter().join(sep)
    }
}

impl<I, S> std::fmt::Display for StringJoiner<I, S> where I: std::iter::Iterator, S: std::fmt::Display, I::Item: std::fmt::Display, I: Clone {
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
