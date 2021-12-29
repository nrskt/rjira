/// It can be found a specific item.
///
/// `Self::Key` means the type used by the search key.
/// `Self::Ret` means the return type of search result.
pub trait FindFromCollection {
    type Key;
    type Ret;

    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;

    /// find a specific item.
    fn find_by_id_mut(&mut self, key: &Self::Key) -> Option<&mut Self::Ret>;
}

#[cfg(test)]
mod tests {
    use std::hash::Hash;

    use indexmap::{indexmap, IndexMap};

    use super::*;

    #[test]
    fn test_find_from_collection() {
        let mut col: IndexMap<u8, String> = indexmap! {
            1 => "test".to_string(),
        };

        assert!(col.find_by_id_mut(&0).is_none());
        assert!(col.find_by_id_mut(&1).is_some());
    }

    impl<Key: Hash + Eq, Val: Clone> FindFromCollection for IndexMap<Key, Val> {
        type Key = Key;
        type Ret = Val;

        fn len(&self) -> usize {
            self.len()
        }

        fn is_empty(&self) -> bool {
            self.is_empty()
        }

        fn find_by_id_mut(&mut self, key: &Self::Key) -> Option<&mut Self::Ret> {
            self.get_mut(key)
        }
    }
}
