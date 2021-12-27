use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    AddItem, AssignableFromCollection, BacklogItem, Entity, EstimatableFromCollection,
    FindFromCollection,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Backlog {
    id: Uuid,
    items: IndexMap<Uuid, Box<dyn BacklogItem>>,
}

impl Backlog {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            items: IndexMap::new(),
        }
    }
}

impl Entity for Backlog {
    fn id(&self) -> Uuid {
        self.id
    }
}

impl AddItem for Backlog {
    fn mut_items(&mut self) -> &mut IndexMap<Uuid, Box<dyn BacklogItem>> {
        &mut self.items
    }
}

impl FindFromCollection for Backlog {
    type Key = Uuid;
    type Ret = Box<dyn BacklogItem>;

    fn find_by_id_mut(&mut self, key: &Self::Key) -> Option<&mut Self::Ret> {
        self.items.get_mut(key)
    }
}

impl AssignableFromCollection for Backlog {}

impl EstimatableFromCollection for Backlog {}
