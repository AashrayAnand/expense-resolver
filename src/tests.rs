#[cfg(test)]
use crate::json::*;

#[test]
pub fn test_serde_budgets() {
    let input = 
        r##"{"data": {
            "budgets": [
            {
                "id": "string",
                "name": "string",
                "last_modified_on": "2022-03-07T03:28:24.521Z",
                "first_month": "string",
                "last_month": "string",
                "date_format": {
                "format": "string"
                },
                "currency_format": {
                "iso_code": "string",
                "example_format": "string",
                "decimal_digits": 0,
                "decimal_separator": "string",
                "symbol_first": true,
                "group_separator": "string",
                "currency_symbol": "string",
                "display_symbol": true
                },
                "accounts": [
                {
                    "id": "string",
                    "name": "string",
                    "type": "checking",
                    "on_budget": true,
                    "closed": true,
                    "note": "string",
                    "balance": 0,
                    "cleared_balance": 0,
                    "uncleared_balance": 0,
                    "transfer_payee_id": "string",
                    "direct_import_linked": true,
                    "direct_import_in_error": true,
                    "deleted": true
                }
                ]
            }
            ],
            "default_budget": {
            "id": "string",
            "name": "string",
            "last_modified_on": "2022-03-07T03:28:24.521Z",
            "first_month": "string",
            "last_month": "string",
            "date_format": {
                "format": "string"
            },
            "currency_format": {
                "iso_code": "string",
                "example_format": "string",
                "decimal_digits": 0,
                "decimal_separator": "string",
                "symbol_first": true,
                "group_separator": "string",
                "currency_symbol": "string",
                "display_symbol": true
            },
            "accounts": [
                {
                "id": "string",
                "name": "string",
                "type": "checking",
                "on_budget": true,
                "closed": true,
                "note": "string",
                "balance": 0,
                "cleared_balance": 0,
                "uncleared_balance": 0,
                "transfer_payee_id": "string",
                "direct_import_linked": true,
                "direct_import_in_error": true,
                "deleted": true
                }
            ]
            }
        }
    }"##.as_bytes();

    let resp_json: BudgetsJson = serde_json::from_slice(&input).unwrap();
}

