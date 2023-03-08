use core::borrow::Borrow;

/// Key equivalence trait.
///
/// This trait allows hash table lookup to be customized.
/// It has one blanket implementation that uses the regular `Borrow` solution,
/// just like `HashMap` and `BTreeMap` do, so that you can pass `&str` to lookup
/// into a map with `String` keys and so on.
///
/// # Contract
///
/// The implementor **must** hash like `Q`, if it is hashable.
pub trait Equivalent<Q: ?Sized> {
    /// Compare self to `key` and return `true` if they are equal.
    fn equivalent(&self, key: &Q) -> bool;
}

impl<K: ?Sized, Q: ?Sized> Equivalent<Q> for K
where
    K: Borrow<Q>,
    Q: Eq,
{
    #[inline]
    fn equivalent(&self, key: &Q) -> bool {
        PartialEq::eq(self.borrow(), key)
    }
}
