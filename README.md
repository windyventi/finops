# FinOps (Rust Edition)

FinOps is a lightweight personal finance service implemented in Rust. It exposes a
JSON API for managing accounts, budgets, and expenses, making it easy to integrate
with modern clients or automation workflows.

## Getting started

### Prerequisites

- [Rust](https://www.rust-lang.org/) 1.74 or newer with `cargo`

### Run the API server

```bash
cargo run
```

The application listens on `http://localhost:3000` by default.

### Available endpoints

| Method | Path | Description |
| ------ | ---- | ----------- |
| `GET` | `/health` | Health check endpoint |
| `GET` | `/accounts` | List all accounts |
| `POST` | `/accounts` | Create a new account |
| `GET` | `/accounts/:id` | Retrieve a single account |
| `PUT` | `/accounts/:id` | Update an account |
| `DELETE` | `/accounts/:id` | Delete an account |
| `GET` | `/budgets` | List all budgets |
| `POST` | `/budgets` | Create a new budget |
| `GET` | `/budgets/:id` | Retrieve a single budget |
| `PUT` | `/budgets/:id` | Update a budget |
| `DELETE` | `/budgets/:id` | Delete a budget |
| `GET` | `/budgets/:id/expenses` | List expenses for a budget |
| `POST` | `/budgets/:id/expenses` | Create a new expense |
| `GET` | `/budgets/:id/summary` | View aggregate spend and utilization for a budget |
| `PUT` | `/budgets/:budget_id/expenses/:id` | Update an expense |
| `DELETE` | `/budgets/:budget_id/expenses/:id` | Delete an expense |

### Request/Response examples

Create an account:

```bash
curl -X POST http://localhost:3000/accounts \
  -H "Content-Type: application/json" \
  -d '{"name":"Checking","owner":"Alex","notes":"Personal"}'
```

Create a budget:

```bash
curl -X POST http://localhost:3000/budgets \
  -H "Content-Type: application/json" \
  -d '{"name":"Groceries","limit_cents":50000}'
```

Create an expense:

```bash
curl -X POST http://localhost:3000/budgets/{budget_id}/expenses \
  -H "Content-Type: application/json" \
  -d '{"description":"Weekly shop","amount_cents":9500}'
```

Filter expenses:

```bash
curl "http://localhost:3000/budgets/{budget_id}/expenses?min_amount=5000&max_amount=20000&q=shop"
```

Fetch a budget summary:

```bash
curl http://localhost:3000/budgets/{budget_id}/summary
```

## Development

FinOps uses an in-memory data store for simplicity. The API is organised with
Axum routers and shared state via `tokio::sync::RwLock`. Tests can be added
as the domain model evolves.

## License

This project remains available under the AGPLv3 license. See [LICENSE](LICENSE).
