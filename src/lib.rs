pub trait StringJoin<I, S> where I: std::iter::Iterator, S: AsRef<str>, I::Item: std::fmt::Display {
    fn join(self, sep: S) -> StringJoiner<I, S>;
}

#[derive(Debug)]
pub struct StringJoiner<I, S> where I: std::iter::Iterator, S: AsRef<str> {
    iter: I,
    sep: S
}

impl<I, S> StringJoiner<I, S> where I: std::iter::Iterator, S: AsRef<str>, I::Item: std::fmt::Display {
    #[inline]
    pub fn into_string(self) -> String {
        let mut buffer = String::new();
        self.write_into(&mut buffer);
        return buffer;
    }

    #[inline]
    pub fn to_string(&self) -> String where I: Clone {
        return format!("{}", self);
    }

    pub fn write_into<W: std::fmt::Write>(mut self, buffer: &mut W) {
        if let Some(first) = self.iter.next() {
            let _ = write!(buffer, "{}", first);
            let sep = self.sep.as_ref();
            while let Some(item) = self.iter.next() {
                let _ = write!(buffer, "{}{}", sep, item);
            }
        }
    }
}

impl<I, S> Into<String> for StringJoiner<I, S> where I: std::iter::Iterator, S: AsRef<str>, I::Item: std::fmt::Display {
    #[inline]
    fn into(self) -> String {
        self.into_string()
    }
}

impl<I, S> Clone for StringJoiner<I, S> where I: std::iter::Iterator, S: AsRef<str>, I::Item: std::fmt::Display, I: Clone, S: Clone {
    #[inline]
    fn clone(&self) -> Self {
        StringJoiner {
            iter: self.iter.clone(),
            sep: self.sep.clone()
        }
    }
}

impl<I, S> StringJoin<I, S> for I where I: std::iter::Iterator, I::Item: std::fmt::Display, S: AsRef<str> {
    #[inline]
    fn join(self, sep: S) -> StringJoiner<I, S> {
        StringJoiner {
            iter: self,
            sep
        }
    }
}

impl<'a, T, S> StringJoin<core::slice::Iter<'a, T>, S> for &'a [T] where T: std::fmt::Display, S: AsRef<str> {
    #[inline]
    fn join(self, sep: S) -> StringJoiner<core::slice::Iter::<'a, T>, S> {
        self.iter().join(sep)
    }
}

impl<I, S> std::fmt::Display for StringJoiner<I, S> where I: std::iter::Iterator, S: AsRef<str>, I::Item: std::fmt::Display, I: Clone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.iter.clone();
        if let Some(first) = iter.next() {
            first.fmt(f)?;
            let sep = self.sep.as_ref();
            while let Some(item) = iter.next() {
                sep.fmt(f)?;
                item.fmt(f)?;
            }
        }
        Ok(())
    }
}
