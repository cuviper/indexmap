#![allow(unsafe_code)]

use super::{Bucket, IndexMapCore};

impl<K, V> IndexMapCore<K, V> {
    pub(crate) fn extract(&mut self) -> ExtractCore<'_, K, V> {
        // SAFETY: We must have consistent lengths to start, so that's a hard assertion.
        // Then the worst `set_len(0)` can do is leak items if `ExtractCore` doesn't drop.
        assert_eq!(self.entries.len(), self.indices.len());
        unsafe {
            self.entries.set_len(0);
        }
        ExtractCore {
            map: self,
            current: 0,
            new_len: 0,
        }
    }
}

pub(crate) struct ExtractCore<'a, K, V> {
    map: &'a mut IndexMapCore<K, V>,
    current: usize,
    new_len: usize,
}

impl<K, V> Drop for ExtractCore<'_, K, V> {
    fn drop(&mut self) {
        let old_len = self.map.indices.len();
        let mut new_len = self.new_len;
        debug_assert!(new_len <= self.current);
        debug_assert!(self.current <= old_len);
        debug_assert!(old_len <= self.map.entries.capacity());

        // SAFETY: We assume `new_len` and `current` were correctly maintained by the iterator.
        // So `entries[new_len..current]` were extracted, but the rest before and after are valid.
        unsafe {
            if new_len == self.current {
                // Nothing was extracted, so any remaining items can be left in place.
                new_len = old_len;
            } else if self.current < old_len {
                // Need to shift the remaining items down.
                let tail_len = old_len - self.current;
                let base = self.map.entries.as_mut_ptr();
                let src = base.add(self.current);
                let dest = base.add(new_len);
                src.copy_to(dest, tail_len);
                new_len += tail_len;
            }
            self.map.entries.set_len(new_len);
        }

        if new_len != old_len {
            // We don't keep track of *which* items were extracted, so reindex everything.
            self.map.rebuild_hash_table();
        }
    }
}

impl<K, V> ExtractCore<'_, K, V> {
    pub(crate) fn extract_if<F>(&mut self, mut pred: F) -> Option<Bucket<K, V>>
    where
        F: FnMut(&mut Bucket<K, V>) -> bool,
    {
        let old_len = self.map.indices.len();
        debug_assert!(old_len <= self.map.entries.capacity());

        let base = self.map.entries.as_mut_ptr();
        while self.current < old_len {
            // SAFETY: We're maintaining both indices within bounds of the original entries, so
            // 0..new_len and current..old_len are always valid items for our Drop to keep.
            unsafe {
                let item = base.add(self.current);
                if pred(&mut *item) {
                    // Extract it!
                    self.current += 1;
                    return Some(item.read());
                } else {
                    // Keep it, shifting it down if needed.
                    if self.new_len != self.current {
                        debug_assert!(self.new_len < self.current);
                        let dest = base.add(self.new_len);
                        item.copy_to_nonoverlapping(dest, 1);
                    }
                    self.current += 1;
                    self.new_len += 1;
                }
            }
        }
        None
    }

    pub(crate) fn remaining(&self) -> usize {
        self.map.indices.len() - self.current
    }
}
