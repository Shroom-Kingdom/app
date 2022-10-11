use bevy::prelude::*;
use enum_iterator::Sequence;
use serde::{Deserialize, Serialize};

#[derive(Clone, Component, Debug, Deserialize, Eq, Hash, PartialEq, Sequence, Serialize)]
pub enum ThemeVariant {
    Plain,
    Underground,
    Water,
    HauntedHouse,
    Castle,
    Woods,
    Desert,
    Snow,
    Airship,
}

impl ThemeVariant {
    pub fn get_name(&self) -> &str {
        match self {
            ThemeVariant::Plain => "plain",
            ThemeVariant::Underground => "underground",
            ThemeVariant::Water => "water",
            ThemeVariant::HauntedHouse => "hauntedhouse",
            ThemeVariant::Castle => "castle",
            ThemeVariant::Woods => "woods",
            ThemeVariant::Desert => "desert",
            ThemeVariant::Snow => "snow",
            ThemeVariant::Airship => "airship",
        }
    }
}
