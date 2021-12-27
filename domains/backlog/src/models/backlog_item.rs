use std::fmt::Debug;

use dyn_clone::{clone_trait_object, DynClone};

use crate::{Assignable, Entity, Estimatable};

pub trait BacklogItem: DynClone + Debug + Send + Sync + Assignable + Estimatable + Entity {}

clone_trait_object!(BacklogItem);
