/////////////////////////////////////////////////////////////////////////////
// json defines all of the structures which we use for de-serializing json results
// from the YNAB API.

/////////////////////////////////////////////////////////////////////////////
use serde::{Deserialize, Serialize};
use serde_json::Value; // lets us parse ynab responses to Value type, using from_str

#[derive(Serialize, Deserialize, Debug)]
pub struct BudgetsJson {
    pub data: BudgetsJsonInner
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BudgetsJsonInner {
    pub budgets: Vec<BudgetJson>,
    pub default_budget: Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BudgetJson {
    pub id: String,
    pub name: String,
    pub last_modified_on: String,
    pub first_month: String,
    pub last_month: String,
    pub date_format: Value,
    pub currency_format: Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionsJson {
    pub data: TransactionsJsonInner,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionsJsonInner {
    pub transactions: Vec<TransactionJson>,
    pub server_knowledge: i64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionJson {
    pub id: Option<String>,
    pub date: Option<String>,
    pub amount: Option<f64>,
    pub memo: Option<String>,
    pub cleared: Option<String>,
    pub approved: Option<bool>,
    pub flag_color: Option<String>,
    pub account_id: Option<String>,
    pub payee_id: Option<String>,
    pub category_id: Option<String>,
    pub transfer_account_id: Option<String>,
    pub transfer_transaction_id: Option<String>,
    pub matched_transaction_id: Option<String>,
    pub import_id: Option<String>,
    pub deleted: Option<bool>,
    pub account_name: Option<String>,
    pub payee_name: Option<String>,
    pub category_name: Option<String>,
}

