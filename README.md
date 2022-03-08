# expense-resolver

auto-resolve shared household expenses, aggregating transactions from our respective **You Need A Budget** account, into a shared, categorized csv

Using [tokio/hyper](https://hyper.rs/) for async http

Budget data is all sourced from the public [YNAB API](https://api.youneedabudget.com/)

### Re-using this resolver

This resolver can be used generically by anyone with a YNAB account(s), simply

1. generate a PAT token for allowing programmatic access to your YNAB account
2. include the PAT (I am using a git-ignored rs file where I created consts for the PATs I am using, advise follow this pattern as seen in main.rs)
4. For each account you are resolving expenses from, add another call of the **run** function in main, with the budget owner, and PAT inputted.
