use std::fmt::Debug;

use dyn_clone::DynClone;

use crate::{Assignable, Entity, Estimatable};

#[typetag::serde(tag = "type")]
pub trait BacklogItem: DynClone + Debug + Send + Sync + Assignable + Estimatable + Entity {}

dyn_clone::clone_trait_object!(BacklogItem);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Story;

    #[test]
    fn test_deserialize() {
        let s = story();
        let string = serde_json::to_string_pretty(&s).unwrap();
        let v: Box<dyn BacklogItem> = serde_json::from_str(&string).unwrap();
        assert_eq!(s.id(), v.id())
    }

    fn story() -> Box<dyn BacklogItem> {
        let story = Story::new("", None, None);
        Box::new(story)
    }
}
