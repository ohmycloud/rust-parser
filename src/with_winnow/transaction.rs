use winnow::prelude::*;
use winnow::{ascii::digit1, ascii::space1, combinator::opt, Parser};
use winnow::combinator::alt;
use winnow::error::{ErrMode, ErrorKind, ParserError};
use winnow::token::take_till;

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

fn parse_transaction_type(input: &mut &str) -> PResult<TransactionType> {
    alt((
        "CREDIT".value(TransactionType::CREDIT),
        "DEBIT".value(TransactionType::DEBIT)
    )).parse_next(input)
}

fn parse_date(input: &mut &str) -> PResult<String> {
    digit1.parse_next(input).map(|s: &str| s.to_string())
}

fn parse_description(input: &mut &str) -> PResult<String> {
    take_till(0..,|c: char| c == '$')
        .parse_next(input)
        .map(|s: &str| s.trim().to_string())
}

fn parse_amount(input: &mut &str) -> PResult<f64> {
    ('$', digit1, opt((".", digit1)))
        .parse_next(input)
        .and_then(|(_, dollars, cents)| {
            let mut amount_str = dollars.to_string();
            if let Some((_, cents)) = cents {
                amount_str.push('.');
                amount_str.push_str(cents);
            }
            amount_str.parse::<f64>().map_err(|_| ErrMode::from_error_kind(input, ErrorKind::Verify))
        })
}

pub fn parse_transaction(input: &mut &str) -> PResult<Transaction> {
    let (transaction_type, _, date, _, description, amount) = (
        parse_transaction_type,
        space1,
        parse_date,
        space1,
        parse_description,
        parse_amount,
    )
        .parse_next(input)?;

    Ok(Transaction {
        transaction_type,
        date,
        description,
        amount,
    })
}