// extern crate google_sheets4 as sheets4;
// use sheets4::api::ValueRange;
// use sheets4::{Result, Error};
// use std::default::Default;
// use sheets4::{Sheets, oauth2, hyper, hyper_rustls};

use std::io::Write;
use std::fs::OpenOptions;
use std::path::PathBuf;
use crate::json::TransactionJson;

// stop gap before there is full automation, just write all of the txns to a csv and we can inspect this
// or upload ourself
pub fn write_sheet(expenses: &mut Vec<TransactionJson>, full_path: &PathBuf, owner: &str) {
    let mut file = OpenOptions::new().create(true).append(true).open(full_path).unwrap();

    for txn in expenses {
        // If a transaction is missing any of the fields we log to expenses, log it and skip
        match (&txn.date, &txn.amount, &txn.account_name, &txn.category_name, &txn.memo) {
            (Some(date), Some(amount), Some(acct), Some(cat), Some(memo)) => {
                let entry = format!("{}, {}, {}, {}, {} {}\n", owner, date, (*amount as f64 / -1000.0), acct, cat, memo);
                match file.write(&entry.as_bytes()) {
                    Err(err) => println!("Failed to write entry {} to file with error {}", entry, err),
                    _ => {}
                }
            }
            _ => {println!("Skipping an expense that's missing some data! {:#?}", txn);}
        }
    }
}