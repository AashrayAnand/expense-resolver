pub mod budget;
pub mod upload;
pub mod tests;
pub mod json;
pub mod pat;
pub mod dbconn;

use std::env::args;

use crate::budget::Budget;
use crate::upload::write_sheet;
use crate::pat::*;
use crate::dbconn::*;
use std::path::PathBuf;

pub async fn run(sqlconn: &SqlConn, full_path: &PathBuf, pat: String, owner: &str) {
    // look up latest server knowledge (if any). This is to ensure we don't repeat
    // query old results from the YNAB API
    let server_knowledge = match sqlconn.get_server_knowledge(owner) {
        Some(knowledge) => knowledge,
        None => 0
    };

    if let Some(mut budget) = Budget::new(pat).await {
        if let Some((mut transactions,knowledge)) = budget.get_transactions(server_knowledge).await {
            write_sheet(&mut transactions, full_path, owner);
            if transactions.len() > 0 {
                sqlconn.insert_txns(owner, transactions);
                sqlconn.set_server_knowledge(owner, knowledge);
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let args: bool = match args().nth(1) {
        Some(flag) => flag == "1",
        _ => true
    };

    let mut full_path = std::env::current_dir().unwrap();
    let file_path = format!("expenses_on_{}.csv", chrono::offset::Utc::now());
    full_path.push(file_path);

    let sqlconn = SqlConn::new();
    sqlconn.warmup_db();

    run(&sqlconn, &full_path, String::from(AAPAT), "Aashray").await;
    run(&sqlconn, &full_path, String::from(LLPAT), "Lizzie").await;

    sqlconn.get_txns();
}