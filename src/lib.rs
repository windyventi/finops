pub mod domain;
pub mod error;
pub mod routes;
pub mod services;
pub mod state;

use axum::Router;

use crate::state::AppState;

pub fn app_router(state: AppState) -> Router<AppState> {
    routes::router().with_state(state)
}
