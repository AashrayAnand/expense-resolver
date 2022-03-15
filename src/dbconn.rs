use crate::json::TransactionJson;
use sqlite::Connection;
use sqlite::State;

pub struct SqlConn {
    conn: Connection
}

impl SqlConn {
    pub fn new() -> SqlConn {
        match sqlite::open("expenses.db") {
            Ok(conn) => {return SqlConn{conn};},
            _ => panic!("unable to open sqlite conn.")
        }
    }

    // Init the DB schema, if not already created
    pub fn warmup_db(&self) {
        self.conn.execute("
                CREATE TABLE IF NOT EXISTS server_knowledge (
                    owner TEXT,
                    knowledge INTEGER);
                CREATE TABLE IF NOT EXISTS expenses (
                    id TEXT NOT NULL PRIMARY KEY,
                    owner TEXT,
                    date TEXT, 
                    amount INTEGER,
                    account TEXT,
                    category TEXT, 
                    memo TEXT);
                ").unwrap();
    }

    pub fn get_txns(&self) {
        let mut statement = self.conn.prepare("
                SELECT id, date, amount, account, category, memo
                FROM expenses
                ORDER BY date DESC
                LIMIT ? ;")
                .unwrap();
        
        statement.bind(1, 20).unwrap();

        while let State::Row = statement.next().unwrap() {
            println!("{}, {}, {}, {}, {}, {}", 
            statement.read::<String>(0).unwrap(),
            statement.read::<String>(1).unwrap(),
            statement.read::<i64>(2).unwrap(),
            statement.read::<String>(3).unwrap(),
            statement.read::<String>(4).unwrap(),
            statement.read::<String>(5).unwrap(),);
        }
    }

    pub fn insert_txns(&self, owner: &str, txns: Vec<TransactionJson>) {
        // accumulate each transaction into a row of the insert statement
        let mut query = txns
            .into_iter()
            .fold(String::from("INSERT OR IGNORE INTO expenses VALUES "), 
            |mut accum: String, txn: TransactionJson| {
                match (txn.id, txn.date,  txn.amount, txn.account_name, txn.category_name, txn.memo) {
                    (Some(id), Some(date), Some(amount), Some(acct), Some(cat), Some(memo)) => {
                        accum.push_str(&format!("\n(\'{}\', \'{}\', \'{}\', {}, \'{}\', \'{}\', \'{}\'),", 
                        id, owner, date, amount, acct, cat, memo));
                    },
                    _ => {}
                }
            accum
        });
        query.replace_range(query.len() - 1..query.len(), ";");

        println!("Query is {}", query);

        self.conn
            .execute(query,)
            .unwrap();
    }
    pub fn set_server_knowledge(&self, owner: &str, latest_knowledge: i64) {

        let mut statement = self.conn.prepare("
        INSERT OR REPLACE INTO server_knowledge(owner, knowledge)
        VALUES (?, ?)
        ").unwrap();

        statement.bind(1, owner).unwrap();
        statement.bind(2, latest_knowledge).unwrap();

        statement.next();
    }

    pub fn get_server_knowledge(&self, owner: &str) -> Option<i64> {

        let mut statement = self.conn.prepare("
        SELECT knowledge
        FROM server_knowledge
        WHERE OWNER = ?
        LIMIT ?
        ").unwrap();

        statement.bind(1, owner).unwrap();
        statement.bind(2, 1).unwrap();

        // Expect only 1 result
        match statement.next() {
            Ok(state) => {
                match state {
                    State::Row => {
                        let server_knowledge = statement.read::<i64>(0).unwrap();
                        println!("server knowledge is {}", server_knowledge);
                        Some(server_knowledge)
                    }
                    _ => None
                }
            }
            _ => {
                // Failed to get server knowledge, or haven't yet queried
                // YNAB API yet
                None
            }
        }
    }
}