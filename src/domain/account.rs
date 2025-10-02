use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Account {
    pub id: Uuid,
    pub name: String,
    pub owner: String,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AccountInput {
    pub name: String,
    pub owner: String,
    pub notes: Option<String>,
}
