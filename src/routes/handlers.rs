use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::{
    domain::{Account, AccountInput, Budget, BudgetInput, BudgetSummary, Expense, ExpenseInput},
    error::ApiError,
    services::analytics,
    state::AppState,
};

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
struct ExpenseFilters {
    pub min_amount: Option<i64>,
    pub max_amount: Option<i64>,
    pub q: Option<String>,
}

pub async fn health() -> &'static str {
    "ok"
}

pub async fn list_accounts(State(state): State<AppState>) -> Json<Vec<Account>> {
    let guard = state.accounts.read().await;
    Json(guard.values().cloned().collect())
}

pub async fn create_account(
    State(state): State<AppState>,
    Json(payload): Json<AccountInput>,
) -> (StatusCode, Json<Account>) {
    let account = Account {
        id: Uuid::new_v4(),
        name: payload.name,
        owner: payload.owner,
        notes: payload.notes,
    };

    state
        .accounts
        .write()
        .await
        .insert(account.id, account.clone());
    (StatusCode::CREATED, Json(account))
}

pub async fn get_account(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Account>, ApiError> {
    let guard = state.accounts.read().await;
    let account = guard.get(&id).cloned().ok_or(ApiError::NotFound)?;
    Ok(Json(account))
}

pub async fn update_account(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<AccountInput>,
) -> Result<Json<Account>, ApiError> {
    let mut guard = state.accounts.write().await;
    let existing = guard.get_mut(&id).ok_or(ApiError::NotFound)?;
    existing.name = payload.name;
    existing.owner = payload.owner;
    existing.notes = payload.notes;
    Ok(Json(existing.clone()))
}

pub async fn delete_account(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    let mut guard = state.accounts.write().await;
    guard.remove(&id).ok_or(ApiError::NotFound)?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn list_budgets(State(state): State<AppState>) -> Json<Vec<Budget>> {
    let guard = state.budgets.read().await;
    Json(guard.values().cloned().collect())
}

pub async fn create_budget(
    State(state): State<AppState>,
    Json(payload): Json<BudgetInput>,
) -> Result<(StatusCode, Json<Budget>), ApiError> {
    if payload.limit_cents < 0 {
        return Err(ApiError::Invalid("limit must be positive".into()));
    }

    let budget = Budget {
        id: Uuid::new_v4(),
        name: payload.name,
        limit_cents: payload.limit_cents,
        notes: payload.notes,
    };

    state
        .budgets
        .write()
        .await
        .insert(budget.id, budget.clone());
    Ok((StatusCode::CREATED, Json(budget)))
}

pub async fn get_budget(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Budget>, ApiError> {
    let guard = state.budgets.read().await;
    let budget = guard.get(&id).cloned().ok_or(ApiError::NotFound)?;
    Ok(Json(budget))
}

pub async fn update_budget(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<BudgetInput>,
) -> Result<Json<Budget>, ApiError> {
    if payload.limit_cents < 0 {
        return Err(ApiError::Invalid("limit must be positive".into()));
    }

    let mut guard = state.budgets.write().await;
    let existing = guard.get_mut(&id).ok_or(ApiError::NotFound)?;
    existing.name = payload.name;
    existing.limit_cents = payload.limit_cents;
    existing.notes = payload.notes;
    Ok(Json(existing.clone()))
}

pub async fn delete_budget(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    let mut guard = state.budgets.write().await;
    guard.remove(&id).ok_or(ApiError::NotFound)?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn list_expenses(
    State(state): State<AppState>,
    Path(budget_id): Path<Uuid>,
    Query(filters): Query<ExpenseFilters>,
) -> Result<Json<Vec<Expense>>, ApiError> {
    {
        let budgets = state.budgets.read().await;
        if !budgets.contains_key(&budget_id) {
            return Err(ApiError::NotFound);
        }
    }

    let normalized_query = filters.q.as_ref().map(|query| query.to_lowercase());

    let mut expenses: Vec<Expense> = {
        let guard = state.expenses.read().await;
        guard
            .values()
            .filter(|expense| expense.budget_id == budget_id)
            .filter(|expense| match filters.min_amount {
                Some(min) => expense.amount_cents >= min,
                None => true,
            })
            .filter(|expense| match filters.max_amount {
                Some(max) => expense.amount_cents <= max,
                None => true,
            })
            .filter(|expense| match normalized_query.as_ref() {
                Some(query) if !query.is_empty() => {
                    expense.description.to_lowercase().contains(query)
                }
                _ => true,
            })
            .cloned()
            .collect()
    };

    expenses.sort_by(|a, b| a.description.cmp(&b.description));
    Ok(Json(expenses))
}

pub async fn create_expense(
    State(state): State<AppState>,
    Path(budget_id): Path<Uuid>,
    Json(payload): Json<ExpenseInput>,
) -> Result<(StatusCode, Json<Expense>), ApiError> {
    if payload.amount_cents <= 0 {
        return Err(ApiError::Invalid("amount must be positive".into()));
    }

    let budgets = state.budgets.read().await;
    if !budgets.contains_key(&budget_id) {
        return Err(ApiError::NotFound);
    }
    drop(budgets);

    let expense = Expense {
        id: Uuid::new_v4(),
        budget_id,
        description: payload.description,
        amount_cents: payload.amount_cents,
    };

    state
        .expenses
        .write()
        .await
        .insert(expense.id, expense.clone());

    Ok((StatusCode::CREATED, Json(expense)))
}

pub async fn update_expense(
    State(state): State<AppState>,
    Path((budget_id, id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<ExpenseInput>,
) -> Result<Json<Expense>, ApiError> {
    if payload.amount_cents <= 0 {
        return Err(ApiError::Invalid("amount must be positive".into()));
    }

    let mut guard = state.expenses.write().await;
    let existing = guard.get_mut(&id).ok_or(ApiError::NotFound)?;

    if existing.budget_id != budget_id {
        return Err(ApiError::NotFound);
    }

    existing.description = payload.description;
    existing.amount_cents = payload.amount_cents;
    Ok(Json(existing.clone()))
}

pub async fn delete_expense(
    State(state): State<AppState>,
    Path((budget_id, id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, ApiError> {
    let mut guard = state.expenses.write().await;
    let existing = guard.get(&id).ok_or(ApiError::NotFound)?;

    if existing.budget_id != budget_id {
        return Err(ApiError::NotFound);
    }

    guard.remove(&id);
    Ok(StatusCode::NO_CONTENT)
}

pub async fn budget_summary(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<BudgetSummary>, ApiError> {
    let summary = analytics::build_budget_summary(&state, id).await?;
    Ok(Json(summary))
}
