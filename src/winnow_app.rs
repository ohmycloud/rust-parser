mod with_winnow;

use winnow::Parser;
use with_winnow::parse_transaction;
use crate::with_winnow::parse_itinerary;

fn main() -> Result<(), winnow::error::ParseError<&'static str, &'static str>> {
    let inputs = [
        "CREDIT    04062020    PayPal transfer    $4.99",
        "CREDIT    04032020    Payroll            $69.73",
        "DEBIT     04022020    ACH transfer       $38.25",
        "DEBIT     03242020    IRS tax kind       $52249.98",
    ];

    for input in inputs.iter() {
        match parse_transaction.parse(input) {
            Ok(transaction) => println!("{:?}", transaction),
            Err(e) => println!("Error parsing '{}': {:?}", input, e),
        }
    }


    let input = r#"Russia
    Vladivostok : 43.131621,131.923828 : 4
    Ulan Ude : 51.841624,107.608101 : 2
    Saint Petersburg : 59.939977,30.315785 : 10
Norway
    Oslo : 59.914289,10.738739 : 2
    Bergen : 60.388533,5.331856 : 4
Ukraine
    Kiev : 50.456001,30.50384 : 3
Switzerland
    Wengen : 46.608265,7.922065 : 3
    Bern : 46.949076,7.448151 : 1"#;

    match parse_itinerary.parse_peek(input) {
        Ok((_, itinerary)) => println!("{:#?}", itinerary),
        Err(e) => eprintln!("Error parsing itinerary: {}", e),
    }

    Ok(())
}
