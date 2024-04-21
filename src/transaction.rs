use std::str::FromStr;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::IResult;
use nom::bytes::complete::take;
use nom::sequence::tuple;
use nom::character::complete;

#[derive(Debug, Eq, PartialEq)]
enum TransactionKind {
    CREDIT, // 信用卡
    DEBIT,  // 银行卡
}

impl FromStr for TransactionKind {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let kind = match s {
            "CREDIT" => TransactionKind::CREDIT,
            "DEBIT" => TransactionKind::DEBIT,
            _ => return Err(format!("{} can't be converted", s))
        };
        Ok(kind)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Date {
    month: u8,
    day: u8,
    year: u16
}

struct Amount {
    value_times100: u8
}

fn parse_credit(input: &str) -> IResult<&str, &str> {
    Ok(("", "CREDIT"))
}

fn parse_debit(input: &str) -> IResult<&str, &str> {
    Ok(("", "DEBIT"))
}

fn parse_day(input: &str) -> IResult<&str, u8> {
    let (input, day) = take(2usize)(input)?;
    let res = complete::u8(day)?;

    Ok((input, res.1))
}

fn parse_month(input: &str) -> IResult<&str, u8> {
    let (input, month) = take(2usize)(input)?;
    let res = complete::u8(month)?;
    Ok((input, res.1))
}

fn parse_year(input: &str) -> IResult<&str, u16> {
    let (input, year) = take(4usize)(input)?;
    let res = complete::u16(year)?;
    Ok((input, res.1))
}

fn parse_mmddyyyy(input: &str) -> IResult<&str, Date> {
    let mut parser = tuple((parse_day, parse_month, parse_year));
    let (input, (day, month, year)) = parser(input)?;
    Ok((input, Date {day, month, year}))
}

fn parse_kind(input: &str) -> IResult<&str, TransactionKind > {
    let (input, kind) = alt((parse_credit, parse_debit))(input)?;
    Ok((input, kind.parse::<TransactionKind>().unwrap()))
}

fn parse_description(input: &str) -> IResult<&str, &str> {
    let (input, description) = take_until("$")(input)?;
    Ok((input, description.trim()))
}

fn main() {
    let input = r#"
    CREDIT    04062020    PayPal transfer    $4.99
    CREDIT    04032020    Payroll            $69.73
    DEBIT     04022020    ACH transfer       $38.25
    DEBIT     03242020    IRS tax kind       $52249.98
    "#.trim_start();

    println!("{}", input);
}

#[test]
fn test_trans_kind() {
    let input = "CREDIT";
    assert_eq!(parse_kind(input), Ok(("", TransactionKind::CREDIT)));
}

#[test]
fn test_day() {
    let input = "04062020";
    assert_eq!(parse_day(input), Ok(("", 4)));
}

#[test]
fn test_mmddyyyy() {
    let input = "04062020";
    assert_eq!(parse_mmddyyyy(input), Ok(("", Date {
        day: 4,
        month: 6,
        year: 2020,
    })));
}

#[test]
fn test_description() {
    let input = "PayPal transfer    $4.99";
    assert_eq!(parse_description(input), Ok(("$4.99", "PayPal transfer")));
}