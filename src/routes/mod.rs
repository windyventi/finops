mod handlers;

use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/health", get(handlers::health))
        .route(
            "/accounts",
            get(handlers::list_accounts).post(handlers::create_account),
        )
        .route(
            "/accounts/:id",
            get(handlers::get_account)
                .put(handlers::update_account)
                .delete(handlers::delete_account),
        )
        .route(
            "/budgets",
            get(handlers::list_budgets).post(handlers::create_budget),
        )
        .route(
            "/budgets/:id",
            get(handlers::get_budget)
                .put(handlers::update_budget)
                .delete(handlers::delete_budget),
        )
        .route(
            "/budgets/:id/expenses",
            get(handlers::list_expenses).post(handlers::create_expense),
        )
        .route("/budgets/:id/summary", get(handlers::budget_summary))
        .route(
            "/budgets/:budget_id/expenses/:id",
            put(handlers::update_expense).delete(handlers::delete_expense),
        )
}
