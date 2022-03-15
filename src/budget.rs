/////////////////////////////////////////////////////////////////////////////
// Budget is a client for getting budget data from YNAB via the public API.

// Hyper (tokio http client) is used for making API calls, all resultant data 
//is offloaded to the upload mod for further writing to sql etc.

/////////////////////////////////////////////////////////////////////////////
use hyper_tls::HttpsConnector;
use hyper::{http::request::Builder, Method, Request, Uri, Client, Body};
use futures::*;
use crate::json::*;

const GET_BUDGETS_URI: &'static str = "https://api.youneedabudget.com/v1/budgets?include_accounts=false";
const EXP_PREFIX: &'static str = "ALSHARED";

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

                // Get most recently edited budget as source for transactions
                let mut budgets: Vec<BudgetJson> = resp_json.data.budgets;
                budgets.sort_by(|a, b| b.last_modified_on.cmp(&a.last_modified_on));
                let active_budget: &BudgetJson = &budgets[0];

                println!("Getting transactions for... \nID -> {} \nName -> {} \nLast Modified -> {} \nFirst Month -> {} \nLast Month -> {}", 
                    active_budget.id, active_budget.name, active_budget.last_modified_on, active_budget.first_month,  active_budget.last_month);

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
    pub async fn get_transactions(&mut self, server_knowedge: i64) -> Option<(Vec<TransactionJson>, i64)> {
        // need to use TLS conn. for YNAB API (Client::new is HTTP)
        let client = Client::builder().build::<_, hyper::Body>(HttpsConnector::new());
        if let Some(txn_req) = self.get_txn_uri(server_knowedge) {
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
                    let transactions_inner: TransactionsJsonInner = transactions_json.data;
                    let transactions: Vec<TransactionJson> = transactions_inner.transactions;
                    let knowledge: i64 = transactions_inner.server_knowledge;

                    let valid_expenses: Vec<TransactionJson> = transactions.into_iter()
                    .filter(|txn| {
                        if txn.amount < Some(0.0) && txn.approved == Some(true) && !txn.category_id.is_none() && !txn.cleared.is_none() {
                            match &txn.memo {
                                Some(memo) => {return memo.contains(EXP_PREFIX)},
                                _ => {}
                            }
                        }
                        return false
                    })
                    .map(|mut txn| {
                        // Remove expense prefix after using it for filtering out non-shared expenses.
                        // Any remaining content in memo is used as the note for the txn

                        // Also divide each transaction amount, they are set oddly in the response

                        // We can unwrap here, verified values are non-null
                        txn.memo = Some(txn.memo.unwrap().split_off(EXP_PREFIX.len()));
                        txn.amount = Some(txn.amount.unwrap() / -1000.0);
                        txn
                    })
                    .collect();

                    println!("There are {} shared expenses since {:#?}", valid_expenses.len(), self.last_month.as_ref().unwrap());

                    return Some((valid_expenses, knowledge));
                }
                Err(err) => {
                    println!("Failed to get budget ID, with error {}", err);
                }
            }
        }
        None
    }

    fn get_txn_uri(&self, server_knowledge: i64) -> Option<Request<Body>> {
        if let Some(id) = &self.budget_id {
            let uri = format!(
                    "https://api.youneedabudget.com/v1/budgets/{}/transactions?last_knowledge_of_server={}", 
            id, server_knowledge).parse().unwrap();

            if let Ok(req) = self.build_req(uri).body(Body::from("")) {
                return Some(req);
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