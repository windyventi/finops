use uuid::Uuid;

use crate::{
    domain::{BudgetSummary, Expense},
    error::ApiError,
    state::AppState,
};

pub async fn build_budget_summary(
    state: &AppState,
    budget_id: Uuid,
) -> Result<BudgetSummary, ApiError> {
    let budget = {
        let guard = state.budgets.read().await;
        guard.get(&budget_id).cloned().ok_or(ApiError::NotFound)?
    };

    let spent_cents = {
        let guard = state.expenses.read().await;
        guard
            .values()
            .filter(|expense| expense.budget_id == budget_id)
            .map(|Expense { amount_cents, .. }| *amount_cents)
            .sum()
    };

    Ok(BudgetSummary::new(budget, spent_cents))
}
