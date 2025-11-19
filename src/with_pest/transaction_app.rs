use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar/transaction.pest"]
pub struct TransactionParser;

fn main() {
    let transaction: &str = r###"CREDIT    04062020    PayPal transfer    $4.99
        CREDIT    04032020    Payroll            $69.73
        DEBIT     04022020    ACH transfer       $38.25
        DEBIT     03242020    IRS tax payment    $52249.98
    "###;

    println!("{:?}", transaction);

    let p = TransactionParser::parse(Rule::transactions, transaction)
        .expect("unsuccessful parse")
        .next()
        .unwrap();
    println!("{:?}", p);
}
