use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use finops::{
    app_router,
    domain::{Budget, BudgetSummary, Expense},
    state::AppState,
};
use http_body_util::BodyExt;
use tower::ServiceExt;
use uuid::Uuid;

async fn read_body_bytes(response: axum::response::Response) -> Vec<u8> {
    response
        .into_body()
        .collect()
        .await
        .expect("body to collect")
        .to_bytes()
        .to_vec()
}

#[tokio::test]
async fn budget_summary_reports_remaining_and_utilization() {
    let state = AppState::default();
    let budget_id = Uuid::new_v4();

    {
        let mut budgets = state.budgets.write().await;
        budgets.insert(
            budget_id,
            Budget {
                id: budget_id,
                name: "Groceries".into(),
                limit_cents: 10_000,
                notes: None,
            },
        );
    }

    {
        let mut expenses = state.expenses.write().await;
        expenses.insert(
            Uuid::new_v4(),
            Expense {
                id: Uuid::new_v4(),
                budget_id,
                description: "Weekly shop".into(),
                amount_cents: 2_500,
            },
        );
        expenses.insert(
            Uuid::new_v4(),
            Expense {
                id: Uuid::new_v4(),
                budget_id,
                description: "Top-up shop".into(),
                amount_cents: 3_000,
            },
        );
    }

    let app = app_router(state);
    let response = app
        .oneshot(
            Request::builder()
                .uri(format!("/budgets/{}/summary", budget_id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let summary: BudgetSummary = serde_json::from_slice(&read_body_bytes(response).await).unwrap();
    assert_eq!(summary.spent_cents, 5_500);
    assert_eq!(summary.remaining_cents, 4_500);
    assert!((summary.utilization - 0.55).abs() < f64::EPSILON);
}

#[tokio::test]
async fn list_expenses_supports_filters() {
    let state = AppState::default();
    let budget_id = Uuid::new_v4();

    {
        let mut budgets = state.budgets.write().await;
        budgets.insert(
            budget_id,
            Budget {
                id: budget_id,
                name: "Household".into(),
                limit_cents: 50_000,
                notes: None,
            },
        );
    }

    {
        let mut expenses = state.expenses.write().await;
        let make_expense = |description: &str, amount_cents: i64| Expense {
            id: Uuid::new_v4(),
            budget_id,
            description: description.into(),
            amount_cents,
        };

        expenses.insert(Uuid::new_v4(), make_expense("Weekly shop", 12_000));
        expenses.insert(Uuid::new_v4(), make_expense("Online shop", 8_000));
        expenses.insert(Uuid::new_v4(), make_expense("Utilities", 5_000));
        expenses.insert(Uuid::new_v4(), make_expense("Coffee", 400));
    }

    let app = app_router(state);
    let response = app
        .oneshot(
            Request::builder()
                .uri(format!(
                    "/budgets/{}/expenses?min_amount=6000&max_amount=15000&q=shop",
                    budget_id
                ))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let expenses: Vec<Expense> = serde_json::from_slice(&read_body_bytes(response).await).unwrap();
    assert_eq!(expenses.len(), 2);
    assert_eq!(expenses[0].description, "Online shop");
    assert_eq!(expenses[1].description, "Weekly shop");
}

#[tokio::test]
async fn list_expenses_returns_not_found_for_unknown_budget() {
    let state = AppState::default();
    let app = app_router(state);
    let response = app
        .oneshot(
            Request::builder()
                .uri(format!("/budgets/{}/expenses", Uuid::new_v4()))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
