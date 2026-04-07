use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub enum TransactionType {
    CREDIT,
    DEBIT,
}

impl Display for TransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransactionType::CREDIT => write!(f, "CREDIT"),
            TransactionType::DEBIT => write!(f, "DEBIT"),
        }
    }
}

impl TryFrom<&str> for TransactionType {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "CREDIT" => Ok(TransactionType::CREDIT),
            "DEBIT" => Ok(TransactionType::DEBIT),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Currency {
    pub codes: String,
    pub quantity: f64,
}

#[derive(Debug, PartialEq)]
pub struct Transaction {
    pub transaction_type: TransactionType,
    pub date: String,
    pub description: String,
    pub amount: Currency,
}
