use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    AddItem, AssignableFromCollection, BacklogItem, Entity, EstimatableFromCollection,
    FindFromCollection,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(getset::Setters), set = "pub")]
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

impl Default for Backlog {
    fn default() -> Self {
        Self::new()
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

    fn len(&self) -> usize {
        self.items.len()
    }

    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    fn find_by_id_mut(&mut self, key: &Self::Key) -> Option<&mut Self::Ret> {
        self.items.get_mut(key)
    }
}

impl AssignableFromCollection for Backlog {}

impl EstimatableFromCollection for Backlog {}

pub trait BacklogFixture {
    fn empty_items() -> Self;
    fn specific_id() -> (Uuid, Self);
}

impl BacklogFixture for Backlog {
    fn empty_items() -> Self {
        Self::new()
    }
    fn specific_id() -> (Uuid, Self) {
        use serde_json::json;
        use std::str::FromStr;

        let item_id = Uuid::from_str("ec1985c0-b7ee-4556-a0d1-461ee9eb754f").unwrap();
        let backlog = json!({
            "id": "e40018e0-f056-4916-89f1-bfec14e1abe2",
            "items": {
                "ec1985c0-b7ee-4556-a0d1-461ee9eb754f": {
                    "type": "Task",
                    "id": "ec1985c0-b7ee-4556-a0d1-461ee9eb754f",
                    "title": "test",
                    "point": null,
                    "assignee": null
                }
            }
        });
        let backlog = serde_json::from_value(backlog).unwrap();
        (item_id, backlog)
    }
}
