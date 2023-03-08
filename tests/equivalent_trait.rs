use indexmap::indexmap;
use indexmap::Equivalent;

use std::hash::Hash;

#[derive(Debug, Hash)]
pub struct Pair<A, B>(pub A, pub B);

impl<A, B, C, D> Equivalent<Pair<&A, &B>> for (C, D)
where
    A: ?Sized,
    B: ?Sized,
    C: Equivalent<A>,
    D: Equivalent<B>,
{
    fn equivalent(&self, key: &Pair<&A, &B>) -> bool {
        self.0.equivalent(key.0) && self.1.equivalent(key.1)
    }
}

#[test]
fn test_lookup() {
    let s = String::from;
    let map = indexmap! {
        (s("a"), s("b")) => 1,
        (s("a"), s("x")) => 2,
    };

    assert!(map.contains_key(&Pair("a", "b")));
    assert!(!map.contains_key(&Pair("b", "a")));
}

#[test]
fn test_string_str() {
    let s = String::from;
    let mut map = indexmap! {
        s("a") => 1, s("b") => 2,
        s("x") => 3, s("y") => 4,
    };

    assert!(map.contains_key("a"));
    assert!(!map.contains_key("z"));
    assert_eq!(map.swap_remove("b"), Some(2));
}
