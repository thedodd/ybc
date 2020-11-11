use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Entry {
    pub completed: bool,
    pub description: String,
    pub editing: bool,
}
