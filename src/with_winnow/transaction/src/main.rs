use transaction_parser::parse_transaction;
use winnow::{LocatingSlice, Parser};

fn main() -> Result<(), winnow::error::ParseError<&'static str, &'static str>> {
    let inputs = [
        "CREDIT    04062020    PayPal transfer    $4.99",
        "CREDIT    04032020    Payroll            $69.73",
        "DEBIT     04022020    ACH transfer       $38.25",
        "DEBIT     03242020    IRS tax kind       $52249.98",
    ];

    for input in inputs.iter() {
        let input = LocatingSlice::new(*input);
        match parse_transaction.parse(input) {
            Ok(transaction) => println!("{:?}", transaction),
            Err(e) => println!("Error parsing '{}': {:?}", input, e),
        }
    }

    Ok(())
}
