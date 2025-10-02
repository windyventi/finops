use serde::{Deserialize, Serialize};

use super::Budget;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BudgetSummary {
    pub budget: Budget,
    pub spent_cents: i64,
    pub remaining_cents: i64,
    pub utilization: f64,
}

impl BudgetSummary {
    pub fn new(budget: Budget, spent_cents: i64) -> Self {
        let remaining_cents = budget.limit_cents - spent_cents;
        let utilization = if budget.limit_cents <= 0 {
            0.0
        } else {
            spent_cents as f64 / budget.limit_cents as f64
        };

        Self {
            budget,
            spent_cents,
            remaining_cents,
            utilization,
        }
    }
}
