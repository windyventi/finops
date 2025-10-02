use std::{collections::BTreeMap, sync::Arc};

use tokio::sync::RwLock;
use uuid::Uuid;

use crate::domain::{Account, Budget, Expense};

#[derive(Clone)]
pub struct AppState {
    pub accounts: Arc<RwLock<BTreeMap<Uuid, Account>>>,
    pub budgets: Arc<RwLock<BTreeMap<Uuid, Budget>>>,
    pub expenses: Arc<RwLock<BTreeMap<Uuid, Expense>>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            accounts: Arc::new(RwLock::new(BTreeMap::new())),
            budgets: Arc::new(RwLock::new(BTreeMap::new())),
            expenses: Arc::new(RwLock::new(BTreeMap::new())),
        }
    }
}
