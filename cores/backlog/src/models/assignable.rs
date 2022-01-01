use uuid::Uuid;

use crate::{Assignee, BacklogError, BacklogItem, BacklogResult, FindFromCollection};

/// Represents that the item will be assigned to someone.
pub trait Assignable {
    /// mutable accessor
    fn mut_assignee(&mut self) -> &mut Option<Assignee>;

    /// assign the item to the specific assignee.
    fn assign(&mut self, assignee: Assignee) {
        *self.mut_assignee() = Some(assignee);
    }
}

/// The collection can search a specific item and assign it to someone.
pub trait AssignableFromCollection:
    FindFromCollection<Key = Uuid, Ret = Box<dyn BacklogItem>>
{
    /// assign the specific item to the assignee.
    fn assign_item(&mut self, id: &Uuid, assignee: Assignee) -> BacklogResult<()> {
        match self.find_by_id_mut(id) {
            None => Err(BacklogError::not_found(format!(
                "BacklogItem, id: {} does not found",
                id
            ))),
            Some(item) => {
                item.assign(assignee);
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod test_assignable {
    use super::*;

    struct TestAssignable {
        assignee: Option<Assignee>,
    }

    impl Assignable for TestAssignable {
        fn mut_assignee(&mut self) -> &mut Option<Assignee> {
            &mut self.assignee
        }
    }

    #[test]
    fn test_assignable() {
        let mut assignable = TestAssignable { assignee: None };
        assignable.assign(Assignee::new("test"));

        assert_eq!(assignable.assignee, Some(Assignee::new("test")));
    }
}

#[cfg(test)]
mod test_assignable_from_collection {
    use super::*;
    use crate::Story;

    #[test]
    fn test_assign() {
        let mut mock = ExpectReturnSome(vec![Box::new(Story::new("", None, None))]);
        let resutl = mock.assign_item(&Uuid::new_v4(), Assignee::new("test"));
        assert!(resutl.is_ok())
    }

    #[test]
    fn test_assign_expect_fail() {
        let mut mock = ExpectReturnNone;
        let resutl = mock.assign_item(&Uuid::new_v4(), Assignee::new("test"));
        assert!(resutl.is_err())
    }

    struct ExpectReturnNone;

    impl FindFromCollection for ExpectReturnNone {
        type Key = Uuid;
        type Ret = Box<dyn BacklogItem>;

        fn len(&self) -> usize {
            todo!()
        }

        fn is_empty(&self) -> bool {
            todo!()
        }

        fn find_by_id_mut(&mut self, _key: &Self::Key) -> Option<&mut Self::Ret> {
            None
        }
    }

    impl AssignableFromCollection for ExpectReturnNone {}

    struct ExpectReturnSome(Vec<Box<dyn BacklogItem>>);

    impl FindFromCollection for ExpectReturnSome {
        type Key = Uuid;
        type Ret = Box<dyn BacklogItem>;

        fn len(&self) -> usize {
            todo!()
        }

        fn is_empty(&self) -> bool {
            todo!()
        }

        fn find_by_id_mut(&mut self, _key: &Self::Key) -> Option<&mut Self::Ret> {
            self.0.get_mut(0)
        }
    }

    impl AssignableFromCollection for ExpectReturnSome {}
}
