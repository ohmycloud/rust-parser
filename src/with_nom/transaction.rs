#![warn(dead_code)]

use nom::bytes::complete::take;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete;
use nom::character::complete::{alpha1, newline, space1};
use nom::error::{Error, ErrorKind};
use nom::multi::{many0, many1};
use nom::number::complete::double;
use nom::sequence::tuple;
use nom::IResult;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct Transaction {
    kind: TransactionKind,
    date: Date,
    description: String,
    amount: f64,
}

#[derive(Debug, Eq, PartialEq)]
enum TransactionKind {
    CREDIT,  // 信用卡
    DEBIT,   // 银行卡
    Unknown, // 未知
}

#[derive(Debug, PartialEq, Eq)]
struct Date {
    month: u8,
    day: u8,
    year: u16,
}

struct Amount {
    value_times100: u8,
}

fn parse_month(input: &str) -> IResult<&str, u8> {
    let (input, month) = take(2usize)(input)?;
    let res = complete::u8(month)?;
    Ok((input, res.1))
}

fn parse_day(input: &str) -> IResult<&str, u8> {
    let (input, day) = take(2usize)(input)?;
    let res = complete::u8(day)?;

    Ok((input, res.1))
}

fn parse_year(input: &str) -> IResult<&str, u16> {
    let (input, year) = take(4usize)(input)?;
    let res = complete::u16(year)?;
    Ok((input, res.1))
}

fn parse_date(input: &str) -> IResult<&str, Date> {
    let mut parser = tuple((parse_month, parse_day, parse_year));
    let (input, (month, day, year)) = parser(input)?;
    Ok((input, Date { month, day, year }))
}

fn parse_kind(input: &str) -> IResult<&str, TransactionKind> {
    let (input, kind) = alpha1(input)?;
    let kind = match kind {
        "CREDIT" => TransactionKind::CREDIT,
        "DEBIT" => TransactionKind::DEBIT,
        _ => TransactionKind::Unknown,
    };

    Ok((input, kind))
}

fn parse_description(input: &str) -> IResult<&str, &str> {
    let (input, description) = take_until("$")(input)?;
    Ok((input, description.trim()))
}

fn parse_amount(input: &str) -> IResult<&str, f64> {
    let mut parser = tuple((tag("$"), double));
    let (input, amount) = parser(input)?;
    Ok((input, amount.1))
}

fn parse_transaction(input: &str) -> IResult<&str, Transaction> {
    let mut parser = tuple((
        many0(newline),
        space1,
        parse_kind,
        space1,
        parse_date,
        space1,
        parse_description,
        parse_amount,
        many0(newline),
    ));
    let (input, (_, _, kind, _, date, _, description, amount, _)) = parser(input)?;

    Ok((
        input,
        Transaction {
            kind,
            date,
            description: description.into(),
            amount,
        },
    ))
}

pub fn parse_transactions(input: &str) -> IResult<&str, Vec<Transaction>> {
    let mut parser = many1(parse_transaction);
    let (input, transactions) = parser(input)?;
    Ok((input, transactions))
}

#[test]
fn test_trans_kind() {
    let input = "CREDIT";
    assert_eq!(parse_kind(input), Ok(("", TransactionKind::CREDIT)));
}

#[test]
fn test_date() {
    let input = "04062020";
    assert_eq!(
        parse_date(input),
        Ok((
            "",
            Date {
                day: 6,
                month: 4,
                year: 2020,
            }
        ))
    );
}

#[test]
fn test_description() {
    let input = "PayPal transfer    $4.99";
    assert_eq!(parse_description(input), Ok(("$4.99", "PayPal transfer")));
}

#[test]
fn test_amount() {
    let input = "$4.99";
    assert_eq!(parse_amount(input), Ok(("", 4.99)));
}

#[test]
fn test_transaction() {
    let input = "    CREDIT    04062020    PayPal transfer    $4.99";
    assert_eq!(
        parse_transaction(input),
        Ok((
            "",
            Transaction {
                kind: TransactionKind::CREDIT,
                date: Date {
                    month: 4,
                    day: 6,
                    year: 2020
                },
                description: "PayPal transfer".into(),
                amount: 4.99,
            }
        ))
    );
}
