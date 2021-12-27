use indexmap::IndexMap;
use uuid::Uuid;

use crate::BacklogItem;

/// The collection can be added to the item.
pub trait AddItem {
    fn mut_items(&mut self) -> &mut IndexMap<Uuid, Box<dyn BacklogItem>>;

    /// Add the specific item.
    fn add_item(&mut self, item: Box<dyn BacklogItem>) {
        self.mut_items().insert(item.id(), item);
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::Story;

    #[test]
    fn test_add_item() {
        let mut collection = TestAddItem {
            collection: IndexMap::new(),
        };
        collection.add_item(Box::new(Story::new("test title", None, None)));
        assert_eq!(collection.collection.len(), 1);
    }

    struct TestAddItem {
        collection: IndexMap<Uuid, Box<dyn BacklogItem>>,
    }

    impl AddItem for TestAddItem {
        fn mut_items(&mut self) -> &mut IndexMap<Uuid, Box<dyn BacklogItem>> {
            &mut self.collection
        }
    }
}
