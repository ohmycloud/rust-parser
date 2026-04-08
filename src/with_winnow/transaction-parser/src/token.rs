#[derive(Debug, PartialEq, Clone)]
pub enum TransactionType {
    CREDIT,
    DEBIT,
}

#[derive(Debug, PartialEq)]
pub struct Transaction {
    pub transaction_type: TransactionType,
    pub date: String,
    pub description: String,
    pub amount: f64,
}
