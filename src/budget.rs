use hyper_tls::HttpsConnector;
use hyper::{http::request::Builder, Method, Request, Uri, Client, Body, Response};
use futures::*;
use crate::json::*;

const GET_BUDGETS_URI: &'static str = "https://api.youneedabudget.com/v1/budgets?include_accounts=false";

#[derive(Debug)]
pub struct Budget {
    token: String, // Personal Access Token
    budget_id: Option<String>, // Nullable ID for the Budget
    name: Option<String>,
    last_modified_on: Option<String>,
    last_month: Option<String>,
    first_month: Option<String>,
}

impl Budget {
    pub async fn new(token: String) -> Option<Budget> {
        let mut budget = Budget {
            token, 
            budget_id: None,
            name: None,
            last_modified_on: None,
            first_month: None,
            last_month: None,
        };

        // need to use TLS conn. for YNAB API (Client::new is HTTP)
        let client = Client::builder().build::<_, hyper::Body>(HttpsConnector::new());

        // Set up Budget ID, need this for getting budget information e.g. transactions list
        match client.request(budget.get_id()).await {
            Ok(mut response) => {
                // Block on getting back all chunks, fold together and get back response string for json serialization
                let resp_slice = response.body_mut()
                .fold(Vec::new(), |mut accum, chunk| async {
                    match chunk {
                        Ok(chunk) => {accum.extend_from_slice(&*chunk);},
                        Err(_) => {}
                    }
                    accum                
                }).await;
                
                // Get list of budgets from response
                let resp_json: BudgetsJson = serde_json::from_slice(&resp_slice).unwrap();
                let mut budgets: Vec<BudgetJson> = resp_json.data.budgets;

                // Get most recently edited budget as source for transactions
                budgets.sort_by(|a, b| b.last_modified_on.cmp(&a.last_modified_on));
                let active_budget: &BudgetJson = &budgets[0];
                println!("Getting transactions for: {} {} {} {} {}", active_budget.id, active_budget.name, active_budget.last_modified_on, active_budget.first_month,  active_budget.last_month);

                // extract misc. members from budget, to use for other request params
                budget.budget_id = Some(String::from(active_budget.id.clone()));
                budget.name = Some(String::from(active_budget.name.clone()));
                budget.last_modified_on = Some(String::from(active_budget.last_modified_on.clone()));
                budget.first_month = Some(String::from(active_budget.first_month.clone()));
                budget.last_month = Some(String::from(active_budget.last_month.clone()));
            },
            Err(err) => {
                println!("Failed to get budget ID, with error {}", err);
                ()
            }
        }
        Some(budget)
    }

    // Get a list of transactions for the budget. 
    // Pre-condition is that we know the budget ID (which we get
    // on class creation, by reading the first budget response from querying the budget list)
    pub async fn get_transactions(&mut self) -> Option<Vec<TransactionJson>> {
        // need to use TLS conn. for YNAB API (Client::new is HTTP)
        let client = Client::builder().build::<_, hyper::Body>(HttpsConnector::new());
        if let Some(txn_req) = self.get_txn_uri() {
            match client.request(txn_req).await {
                Ok(mut response) => {
                    // Block on getting back all chunks, fold together and get back response string for json serialization
                    let resp_slice = response.body_mut()
                    .fold(Vec::new(), |mut accum, chunk| async {
                        match chunk {
                            Ok(chunk) => {accum.extend_from_slice(&*chunk);},
                            Err(_) => {}
                        }
                        accum                
                    }).await;

                    let transactions_json: TransactionsJson = serde_json::from_slice(&resp_slice).unwrap();
                    let transactions: Vec<TransactionJson> = transactions_json.data.transactions;
                    println!("There are {} transactions since {:#?}", transactions.len(), self.last_month);

                    let valid_expenses: Vec<TransactionJson> = transactions.into_iter()
                    .filter(|txn| {
                        txn.amount < Some(0) && 
                        txn.approved == Some(true) &&
                        !txn.category_id.is_none() &&
                        !txn.cleared.is_none()
                        // TODO: start filtering by the memo here, once we start using one for each txn
                        // && !txn.memo.is_none() && txn.memo() == Some("AL EXPENSE");
                        // we are shared expensing
                    })
                    .collect();

                    return Some(valid_expenses);
                }
                Err(err) => {
                    println!("Failed to get budget ID, with error {}", err);
                }
            }
        }
        None
    }

    fn get_txn_uri(&self) -> Option<Request<Body>> {
        if let Some(id) = &self.budget_id {
            if let Some(last_month) = &self.last_month {
                let uri = 
                    format!("https://api.youneedabudget.com/v1/budgets/{}/transactions?since_date={}", 
                    id, last_month).parse().unwrap();

                if let Ok(req) = self.build_req(uri).body(Body::from("")) {
                    return Some(req);
                }
            }
        }
        None
    }

    fn get_id(&self) -> Request<Body> {
        let uri = GET_BUDGETS_URI.parse().unwrap();
        self.build_req(uri).body(Body::from("")).unwrap()
    }

    // Build generic YNAB request for this budget, using the corresponding token
    // and provided URI. Caller can append any further headers as needed
    pub fn build_req(&self, uri: Uri) -> Builder {
        Request::builder()
                .method(Method::GET)
                .uri(uri)
                .header("content-type", "application/json")
                .header("Authorization", format!("Bearer {}", self.token))    
    }
}

// A function which takes a closure and returns an `i32`.
fn apply_to_3<F>(f: F) -> i32 where
    // The closure takes an `i32` and returns an `i32`.
    F: Fn(i32) -> i32 {

    f(3)
}
