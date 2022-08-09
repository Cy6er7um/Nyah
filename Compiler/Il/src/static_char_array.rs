use serde::{Serialize, Deserialize};

pub type StaticCARef = u32;

#[derive(Clone, Serialize, Deserialize)]
pub struct StaticCA {
    pub data: String,
}
