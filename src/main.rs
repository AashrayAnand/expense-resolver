pub mod budget;
pub mod sheets;
pub mod tests;
pub mod json;
pub mod pat;

use chrono::{NaiveDateTime, Utc, DateTime};

use crate::budget::Budget;
use crate::sheets::write_sheet;
use crate::pat::*;

pub async fn run(f_path: &str, pat: String, owner: &str) {
    if let Some(mut budget) = Budget::new(pat).await {
        println!("{}'s Budget is set up, getting transactions", owner);
        if let Some(mut transactions) = budget.get_transactions().await {
            println!("Got {}'s Transactions", owner);
            write_sheet(&mut transactions, &f_path, owner);
        }
    }
}

#[tokio::main]
async fn main() {
    let f_path = format!("expenses_on_{}.csv", DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(61, 0), Utc));
    run(&f_path, String::from(AAPAT), "Aashray").await;
    run(&f_path, String::from(LLPAT), "Lizzie").await;
}