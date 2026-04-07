// use pest::Parser;
// use pest_derive::Parser;
use transaction_lang::parse_transactions;

fn main() {
    let transaction: &str = r###"CREDIT    04062020    PayPal transfer    $4.99
        CREDIT    04032020    Payroll            $69.73
        DEBIT     04022020    ACH transfer       $38.25
        DEBIT     03242020    IRS tax payment    $52249.98
    "###;

    println!("{:?}", transaction);

    let p = parse_transactions(transaction).expect("unsuccessful parse");

    println!("{:?}", p);
}
