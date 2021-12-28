use serde::{Deserialize, Serialize};

use crate::{BacklogError, BacklogResult};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Title(String);

impl Title {
    pub fn new(title: &str) -> Self {
        Self(title.to_string())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Assignee(String);

impl Assignee {
    pub fn new(name: &str) -> Self {
        Self(name.to_string())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct StoryPoint(u8);

impl StoryPoint {
    pub fn new(point: impl Into<u8>) -> BacklogResult<Self> {
        let point = point.into();
        match &point {
            1 | 2 | 3 | 5 | 8 | 13 | 21 => Ok(Self(point)),
            _ => Err(BacklogError::type_error(
                "StoryPoint must take 1 to 21 fibonacci number",
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest(
        input,
        expected,
        case(0, false),
        case(1, true),
        case(21, true),
        case(22, false)
    )]
    fn test_story_point(input: u8, expected: bool) {
        let point = StoryPoint::new(input);
        assert_eq!(point.is_ok(), expected)
    }
}
