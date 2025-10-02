use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Budget {
    pub id: Uuid,
    pub name: String,
    pub limit_cents: i64,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct BudgetInput {
    pub name: String,
    pub limit_cents: i64,
    pub notes: Option<String>,
}
