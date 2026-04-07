use pest::Parser;
use pest::error::Error;
use pest_derive::Parser;

use crate::{Currency, Transaction, TransactionType};

#[derive(Parser)]
#[grammar = "../../grammar/transaction.pest"]
pub struct TransactionParser;

type Result<T> = std::result::Result<T, Error<Rule>>;

fn parse_transaction_type(input: &str) -> Result<TransactionType> {
    let pair = TransactionParser::parse(Rule::kind, input)?.next().unwrap();
    match pair.as_str() {
        "CREDIT" => Ok(TransactionType::CREDIT),
        "DEBIT" => Ok(TransactionType::DEBIT),
        _ => Ok(TransactionType::DEBIT),
    }
}

fn parse_date(input: &str) -> Result<String> {
    let pair = TransactionParser::parse(Rule::date, input)?.next().unwrap();
    Ok(pair.as_str().to_string())
}

fn parse_description(input: &str) -> Result<String> {
    let pair = TransactionParser::parse(Rule::description, input)?
        .next()
        .unwrap();
    Ok(pair.as_str().to_string())
}

fn parse_currency_symbol(input: &str) -> Result<String> {
    let pair = TransactionParser::parse(Rule::currency_symbol, input)?
        .next()
        .unwrap();
    Ok(pair.as_str().to_string())
}

fn parse_quantity(input: &str) -> Result<f64> {
    let pair = TransactionParser::parse(Rule::quantity, input)?
        .next()
        .unwrap();
    Ok(pair.as_str().parse::<f64>().unwrap())
}

fn parse_amount(input: &str) -> Result<Currency> {
    let mut pairs = TransactionParser::parse(Rule::amount, input)?;
    let mut inner = pairs.next().unwrap().into_inner();

    Ok(Currency {
        codes: inner.next().unwrap().as_str().to_string(),
        quantity: inner.next().unwrap().as_str().parse::<f64>().unwrap(),
    })
}

fn parse_transaction(input: &str) -> Result<Transaction> {
    let mut pairs = TransactionParser::parse(Rule::transaction, input)?;
    let mut inner = pairs.next().unwrap().into_inner();

    let transaction_type = inner.next().unwrap().as_str();
    let date = inner.next().unwrap().as_str().to_string();
    let description = inner.next().unwrap().as_str().to_string();
    let amount = inner.next().unwrap();

    let mut pairs = TransactionParser::parse(Rule::amount, amount.as_str())?;
    let mut inner = pairs.next().unwrap().into_inner();

    let amount = Currency {
        codes: inner.next().unwrap().as_str().to_string(),
        quantity: inner.next().unwrap().as_str().parse::<f64>().unwrap(),
    };

    Ok(Transaction {
        transaction_type: transaction_type.try_into().unwrap(),
        date,
        description,
        amount,
    })
}

pub fn parse_transactions(input: &str) -> Result<Vec<Transaction>> {
    let pairs = TransactionParser::parse(Rule::transactions, input)?;
    let mut transactions = Vec::new();
    for pair in pairs {
        println!("{:?}", pair);
        let mut cursor = pair.as_str();
        let transaction_type = parse_transaction_type(cursor)?;
        let date = parse_date(cursor)?;
        let description = parse_description(&mut cursor)?;
        let amount = parse_amount(cursor)?;
        transactions.push(Transaction {
            transaction_type,
            date,
            description,
            amount,
        });
    }
    Ok(transactions)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("CREDIT", TransactionType::CREDIT)]
    #[case("DEBIT", TransactionType::DEBIT)]
    fn test_parse_transaction_type(#[case] input: &str, #[case] expected: TransactionType) {
        assert_eq!(parse_transaction_type(input).unwrap(), expected)
    }

    #[rstest]
    #[case("20230101", "20230101")]
    fn test_parse_date(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(parse_date(input).unwrap(), expected)
    }

    #[rstest]
    #[case("EU Tax Authority", "EU Tax Authority")]
    fn test_parse_description(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(parse_description(input).unwrap(), expected)
    }

    #[rstest]
    #[case("€", "€")]
    #[case("¥", "¥")]
    #[case("£", "£")]
    #[case("$", "$")]
    fn test_parse_symbol(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(parse_currency_symbol(input).unwrap(), expected)
    }

    #[rstest]
    #[case("100.00", 100.00)]
    #[case("2,475.00", 2475.00)]
    fn test_parse_quantity(#[case] input: &str, #[case] expected: f64) {
        assert_eq!(parse_quantity(input).unwrap(), expected)
    }

    #[rstest]
    #[case("€100.00", Currency { codes: "€".to_string(), quantity: 100.00 })]
    fn test_parse_amount(#[case] input: &str, #[case] expected: Currency) {
        assert_eq!(parse_amount(input).unwrap(), expected)
    }

    #[rstest]
    #[case("CREDIT    04062020    PayPal transfer    $4.99", Transaction {
        transaction_type: TransactionType::CREDIT,
        date: "04062020".to_string(),
        description: "PayPal transfer".to_string(),
        amount: Currency { codes: "$".to_string(), quantity: 4.99 },
    })]
    fn test_parse_transaction(#[case] input: &str, #[case] expected: Transaction) {
        assert_eq!(parse_transaction(input).unwrap(), expected)
    }

    #[rstest]
    #[case("CREDIT    04062020    PayPal transfer    $4.99", vec![Transaction {
        transaction_type: TransactionType::CREDIT,
        date: "04062020".to_string(),
        description: "PayPal transfer".to_string(),
        amount: Currency { codes: "$".to_string(), quantity: 4.99 },
    }])]
    fn test_parse_transactions(#[case] input: &str, #[case] expected: Vec<Transaction>) {
        assert_eq!(parse_transactions(input).unwrap(), expected)
    }
}
