use uuid::Uuid;

use crate::{Assignable, Assignee, BacklogItem, Entity, Estimatable, StoryPoint, Title};

/// It means the task.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Task {
    id: Uuid,
    title: Title,
    point: Option<StoryPoint>,
    assignee: Option<Assignee>,
}

impl Task {
    pub fn new(title: &str, point: Option<StoryPoint>, assignee: Option<Assignee>) -> Self {
        Self {
            id: Uuid::new_v4(),
            title: Title::new(title),
            point,
            assignee,
        }
    }
}

impl BacklogItem for Task {}

impl Entity for Task {
    fn id(&self) -> Uuid {
        self.id
    }
}

impl Assignable for Task {
    fn mut_assignee(&mut self) -> &mut Option<Assignee> {
        &mut self.assignee
    }
}

impl Estimatable for Task {
    fn mut_point(&mut self) -> &mut Option<StoryPoint> {
        &mut self.point
    }
}
