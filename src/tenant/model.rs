use serde_derive::{Deserialize, Serialize};

pub type ID = String;

#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Hash, Eq)]
pub struct Tenant {
    pub id: ID,
    pub name: String,
}
