use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Expense {
    pub id: Uuid,
    pub budget_id: Uuid,
    pub description: String,
    pub amount_cents: i64,
}

#[derive(Debug, Deserialize)]
pub struct ExpenseInput {
    pub description: String,
    pub amount_cents: i64,
}
