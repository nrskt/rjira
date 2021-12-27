use uuid::Uuid;

use crate::{Assignable, Assignee, BacklogItem, Entity, Estimatable, StoryPoint, Title};

/// It means the user story.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(test, derive(getset::Setters), set = "pub")]
pub struct Story {
    id: Uuid,
    title: Title,
    point: Option<StoryPoint>,
    assignee: Option<Assignee>,
}

impl Story {
    pub fn new(title: &str, point: Option<StoryPoint>, assignee: Option<Assignee>) -> Self {
        Self {
            id: Uuid::new_v4(),
            title: Title::new(title),
            point,
            assignee,
        }
    }
}

impl BacklogItem for Story {}

impl Entity for Story {
    fn id(&self) -> Uuid {
        self.id
    }
}

impl Assignable for Story {
    fn mut_assignee(&mut self) -> &mut Option<Assignee> {
        &mut self.assignee
    }
}

impl Estimatable for Story {
    fn mut_point(&mut self) -> &mut Option<StoryPoint> {
        &mut self.point
    }
}
