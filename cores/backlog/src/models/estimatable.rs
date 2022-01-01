use uuid::Uuid;

use crate::{BacklogError, BacklogItem, BacklogResult, FindFromCollection, StoryPoint};

/// It can be estimated by story point.
pub trait Estimatable {
    fn mut_point(&mut self) -> &mut Option<StoryPoint>;

    /// estimate it.
    fn estimate(&mut self, point: StoryPoint) {
        *self.mut_point() = Some(point);
    }
}

/// The collection can search a specific item and estimate it.
pub trait EstimatableFromCollection:
    FindFromCollection<Key = Uuid, Ret = Box<dyn BacklogItem>>
{
    /// estimate the specific item.
    fn estimate_item(&mut self, id: &Uuid, point: StoryPoint) -> BacklogResult<()> {
        match self.find_by_id_mut(id) {
            None => Err(BacklogError::not_found(format!(
                "BacklogItem, id: {} does not found",
                id
            ))),
            Some(item) => {
                item.estimate(point);
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod test_estimatable {
    use super::*;

    #[derive(Debug, Clone)]
    struct TestEstimateable {
        point: Option<StoryPoint>,
    }

    impl Estimatable for TestEstimateable {
        fn mut_point(&mut self) -> &mut Option<StoryPoint> {
            &mut self.point
        }
    }

    #[test]
    fn test_estimatable() {
        let mut estimatable = TestEstimateable { point: None };
        estimatable.estimate(StoryPoint::new(2).unwrap());
        assert_eq!(estimatable.point, Some(StoryPoint::new(2).unwrap()))
    }
}

#[cfg(test)]
mod test_estimatable_from_collection {
    use super::*;
    use crate::Story;

    #[test]
    fn test_estimate() {
        let mut mock = ExpectReturnSome(vec![Box::new(Story::new("", None, None))]);
        let resutl = mock.estimate_item(&Uuid::new_v4(), StoryPoint::new(1).unwrap());
        assert!(resutl.is_ok())
    }

    #[test]
    fn test_estimate_expect_fail() {
        let mut mock = ExpectReturnNone;
        let resutl = mock.estimate_item(&Uuid::new_v4(), StoryPoint::new(1).unwrap());
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

    impl EstimatableFromCollection for ExpectReturnNone {}

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

    impl EstimatableFromCollection for ExpectReturnSome {}
}
