use winnow::combinator::alt;
use winnow::error::{AddContext, ContextError, ErrMode, StrContext};
use winnow::prelude::*;
use winnow::stream::Stream;
use winnow::token::take_till;
use winnow::{Parser, ascii::digit1, ascii::space1, combinator::opt};

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

fn parse_transaction_type(input: &mut &str) -> ModalResult<TransactionType> {
    alt((
        "CREDIT".value(TransactionType::CREDIT),
        "DEBIT".value(TransactionType::DEBIT),
    ))
    .parse_next(input)
}

fn parse_date(input: &mut &str) -> ModalResult<String> {
    digit1.parse_next(input).map(|s: &str| s.to_string())
}

fn parse_description(input: &mut &str) -> ModalResult<String> {
    take_till(0.., |c: char| c == '$')
        .parse_next(input)
        .map(|s: &str| s.trim().to_string())
}

fn parse_amount(input: &mut &str) -> ModalResult<f64> {
    ('$', digit1, opt((".", digit1)))
        .parse_next(input)
        .and_then(|(_, dollars, cents)| {
            let mut amount_str = dollars.to_string();
            if let Some((_, cents)) = cents {
                amount_str.push('.');
                amount_str.push_str(cents);
            }
            amount_str.parse::<f64>().map_err(|_| {
                ErrMode::Cut(ContextError::new().add_context(
                    input,
                    &input.checkpoint(),
                    StrContext::Label("Invalid number format"),
                ))
            })
        })
}

pub fn parse_transaction(input: &mut &str) -> ModalResult<Transaction> {
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
