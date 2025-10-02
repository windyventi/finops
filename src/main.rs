use finops::{app_router, state::AppState};

#[tokio::main]
async fn main() {
    let state = AppState::default();
    let app = app_router(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("failed to bind listener");
    println!(
        "FinOps service running on http://{}",
        listener.local_addr().expect("listener address")
    );

    axum::serve(listener, app).await.expect("server exited");
}
