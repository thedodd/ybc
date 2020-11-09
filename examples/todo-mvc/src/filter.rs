use serde::{ Deserialize, Serialize };
use strum_macros::{ EnumIter, ToString };

use crate::entry::Entry;

#[derive(Clone, Copy, Deserialize, EnumIter, PartialEq, Serialize, ToString)]
pub enum Filter {
    All,
    Active,
    Completed,
}

impl Filter {
    pub fn fits(&self, entry: &Entry) -> bool {
        match *self {
            Filter::All => true,
            Filter::Active => !entry.completed,
            Filter::Completed => entry.completed,
        }
    }

    pub fn as_href(&self) -> &'static str {
        match self {
            Filter::All => "#/",
            Filter::Active => "#/active",
            Filter::Completed => "#/completed",
        }
    }
}