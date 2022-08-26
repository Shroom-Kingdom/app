use bevy::prelude::*;
use enum_iterator::Sequence;

#[derive(Clone, Component, Debug, Eq, Hash, PartialEq, Sequence)]
pub enum ObjectVariant {
    GoalPoleL,
    GoalPoleR,
    GoalPole,
}

impl ObjectVariant {
    pub fn get_name(&self) -> &str {
        match self {
            Self::GoalPoleL => "goalpole_goalpoaltopL",
            Self::GoalPoleR => "goalpole_goalpoaltopR",
            Self::GoalPole => "goalpole_pole",
        }
    }
}
