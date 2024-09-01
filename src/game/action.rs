use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Action {
    pub row: usize,
    pub col: usize,
    pub piece_index: Option<usize>,
}

impl Action {
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}