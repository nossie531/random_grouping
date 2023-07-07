/// Sized Iterator.
///
/// Iterator adapter that complements [`size_hint`](Iterator::size_hint). This
/// adapter use the original iterator if the original iterator has upper bound
/// of `size_hint` already. Otherwise, this adapter collects all elements from
/// the original iterator to create a vector and generates an iterator to it.
/// With this, `size_hint` upper bound is always available in both cases.
pub struct SizedIter<'a, T> {
    src_iter: Option<&'a mut dyn Iterator<Item = T>>,
    vec_iter: Option<<Vec<T> as IntoIterator>::IntoIter>,
}

impl<'a, T> SizedIter<'a, T> {
    /// Create an instance from original iterator.
    pub fn new<I>(iter: &'a mut I) -> Self
    where
        I: Iterator<Item = T>,
    {
        if iter.size_hint().1.is_some() {
            Self {
                src_iter: Some(iter),
                vec_iter: None,
            }
        } else {
            let iter = iter.collect::<Vec<_>>().into_iter();
            Self {
                src_iter: None,
                vec_iter: Some(iter),
            }
        }
    }
}

impl<'a, T> Iterator for SizedIter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.src_iter.is_some() {
            self.src_iter.as_mut().unwrap().next()
        } else if self.vec_iter.is_some() {
            self.vec_iter.as_mut().unwrap().next()
        } else {
            panic!();
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.src_iter.is_some() {
            self.src_iter.as_ref().unwrap().size_hint()
        } else if self.vec_iter.is_some() {
            self.vec_iter.as_ref().unwrap().size_hint()
        } else {
            panic!();
        }
    }
}
