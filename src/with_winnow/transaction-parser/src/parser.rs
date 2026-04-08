use crate::Transaction;
use crate::token::TransactionType;
use winnow::ascii::space0;
use winnow::combinator::alt;
use winnow::error::{AddContext, ContextError, ErrMode, StrContext};
use winnow::prelude::*;
use winnow::stream::Stream;
use winnow::token::take_till;
use winnow::{LocatingSlice, Parser, ascii::digit1, ascii::space1, combinator::opt};

type Input<'a> = LocatingSlice<&'a str>;

fn parse_transaction_type<'a>(input: &mut Input<'a>) -> ModalResult<TransactionType> {
    alt((
        "CREDIT".value(TransactionType::CREDIT),
        "DEBIT".value(TransactionType::DEBIT),
    ))
    .parse_next(input)
}

fn parse_date<'a>(input: &mut Input<'a>) -> ModalResult<String> {
    digit1.parse_next(input).map(|s: &str| s.to_string())
}

fn parse_description<'a>(input: &mut Input<'a>) -> ModalResult<String> {
    take_till(0.., |c: char| matches!(c, '$' | '€' | '£' | '¥'))
        .parse_next(input)
        .map(|s: &str| s.trim().to_string())
}

fn parse_amount<'a>(input: &mut Input<'a>) -> ModalResult<f64> {
    (alt(('$', '€', '£', '¥')), digit1, opt((".", digit1)))
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

pub fn parse_transaction<'a>(input: &mut Input<'a>) -> ModalResult<Transaction> {
    let (_, transaction_type, _, date, _, description, amount) = (
        space0,
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

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("CREDIT", TransactionType::CREDIT)]
    #[case("DEBIT", TransactionType::DEBIT)]
    fn test_parse_trans_kind(#[case] input: String, #[case] expected: TransactionType) {
        let mut input = LocatingSlice::new(input.as_str());
        let transaction_kind = parse_transaction_type(&mut input).unwrap();
        assert_eq!(transaction_kind, expected);
    }

    #[rstest]
    #[case("20230101", "20230101")]
    fn test_parse_date(#[case] input: String, #[case] expected: String) {
        let mut input = LocatingSlice::new(input.as_str());
        let date = parse_date(&mut input).unwrap();
        assert_eq!(date, expected);
    }

    #[rstest]
    #[case("PayPal transfer    $4.99", "PayPal transfer")]
    #[case("Amazon purchase   €850.00", "Amazon purchase")]
    #[case("Barclays Payroll   £475.00", "Barclays Payroll")]
    #[case("微信支付           ¥1280.00", "微信支付")]
    fn test_parse_description(#[case] input: String, #[case] expected: String) {
        let mut input = LocatingSlice::new(input.as_str());
        let description = parse_description(&mut input).unwrap();
        assert_eq!(description, expected);
    }

    #[rstest]
    #[case("$4.99", 4.99)]
    #[case("€850.00", 850.00)]
    #[case("£500.00", 500.00)]
    #[case("¥1000.00", 1000.00)]
    fn test_parse_amount(#[case] input: String, #[case] expected: f64) {
        let mut input = LocatingSlice::new(input.as_str());
        let amount = parse_amount(&mut input).unwrap();
        assert_eq!(amount, expected);
    }

    #[rstest]
    #[case("    CREDIT    04062020    PayPal transfer    $4.99", Transaction {
        transaction_type: TransactionType::CREDIT,
        date: "04062020".to_string(),
        description: "PayPal transfer".to_string(),
        amount: 4.99,
    })]
    #[case("    DEBIT    04062020    Amazon purchase    €850.00", Transaction {
        transaction_type: TransactionType::DEBIT,
        date: "04062020".to_string(),
        description: "Amazon purchase".to_string(),
        amount: 850.00,
    })]
    fn test_parse_transaction(#[case] input: String, #[case] expected: Transaction) {
        let mut input = LocatingSlice::new(input.as_str());
        let transaction = parse_transaction(&mut input).unwrap();
        assert_eq!(transaction, expected);
    }
}
