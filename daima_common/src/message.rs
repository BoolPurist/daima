use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct InitMessage {
    name: String,
}

impl InitMessage {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct UnkownMessage;
