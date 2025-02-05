
mod compute;
mod strategy;

struct Transaction {
    transaction_type: &'static str,
    amount: f64,
}
fn main() {


    

    let transactions = vec![
        Transaction { transaction_type: "Deposit", amount: 100.0 },
        Transaction { transaction_type: "Withdrawal", amount: 50.0 },
        Transaction { transaction_type: "Deposit", amount: 200.0 },
        Transaction { transaction_type: "Withdrawal", amount: 75.0 },
    ];

    let initial_balance = 0.0; // 初始余额为零

    let balance = transactions.iter().fold(initial_balance, |acc, transaction| {
        match transaction.transaction_type {
            "Deposit" => acc + transaction.amount,
            "Withdrawal" => acc - transaction.amount,
            _ => acc,
        }
    });

    println!("Account Balance: ${:.2}", balance);
}
