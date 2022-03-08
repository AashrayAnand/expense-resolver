// extern crate google_sheets4 as sheets4;
// use sheets4::api::ValueRange;
// use sheets4::{Result, Error};
// use std::default::Default;
// use sheets4::{Sheets, oauth2, hyper, hyper_rustls};

use std::io::Write;
use std::fs::OpenOptions;

use crate::json::TransactionJson;

// stop gap before there is full automation, just write all of the txns to a csv and we can inspect this
// or upload ourself
pub fn write_sheet(expenses: &mut Vec<TransactionJson>, f_path: &str, owner: &str) {
    let mut file = OpenOptions::new().create(true).append(true).open(f_path).unwrap();

    for txn in expenses {
        // If a transaction is missing any of the fields we log to expenses, log it and skip
        match (&txn.date, &txn.amount, &txn.account_name, &txn.category_name) {
            (Some(date), Some(amount), Some(acct), Some(cat)) => {
                let entry = format!("{}, {}, {}, {}, {}\n", owner, date, (*amount as f64 / -1000.0), acct, cat);
                match file.write(&entry.as_bytes()) {
                    Ok(_) => print!("{}", entry),
                    Err(err) => println!("Failed to write entry {} to file with error {}", entry, err)
                }
            }
            _ => {println!("Skipping an expense that's missing some data! {:#?}", txn);}
        }
    }
}
/*
pub fn update_sheet(expenses: &Vec<TransactionJson>) {
    let secret: oauth2::ApplicationSecret = Default::default();
    let auth = oauth2::InstalledFlowAuthenticator::builder(
        secret,
        oauth2::InstalledFlowReturnMethod::HTTPRedirect,
    ).build().await.unwrap();
    let mut hub = Sheets::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
    let mut req = ValueRange::default();

    // You can configure optional parameters by calling the respective setters at will, and
    // execute the final call using `doit()`.
    // Values shown here are possibly random and not representative !
    let result = hub.spreadsheets().values_append(req, "spreadsheetId", "range")
            .value_input_option("no")
             .response_value_render_option("ipsum")
             .response_date_time_render_option("voluptua.")
             .insert_data_option("At")
             .include_values_in_response(false)
             .doit().await;
}
*/