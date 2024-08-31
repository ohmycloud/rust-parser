mod with_winnow;

use winnow::Parser;
use with_winnow::parse_transaction;
use crate::with_winnow::{parse_itinerary, parse_log, parse_weather};

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

    let mut input = "2024-08-26 00:00:00.720  [117.132.198.154:3649#172.19.85.132:5003] R:68125e48a4000d0103000300c042009cc4de4200";
    let result = parse_log(&mut input);
    if let Ok(parsed) = result {
        println!("{:#?}", parsed);
    }

    let mut input = r#"
    Name= Jan Mayen
    Country = NORWAY
    Lat=   70.9
    Long=    8.7
    Height= 10
    Start year= 1921
    End year= 2009
    Obs:
    1921 -4.4 -7.1 -6.8 -4.3 -0.8  2.2  4.7  5.8  2.7 -2.0 -2.1 -4.0
    1922 -0.9 -1.7 -6.2 -3.7 -1.6  2.9  4.8  6.3  2.7 -0.2 -3.8 -2.6
    2008 -2.8 -2.7 -4.6 -1.8  1.1  3.3  6.1  6.9  5.8  1.2 -3.5 -0.8
    2009 -2.3 -5.3 -3.2 -1.6  2.0  2.9  6.7  7.2  3.8  0.6 -0.3 -1.3"#;

    if let Ok(x) = parse_weather(&mut input) {
        println!("{:?}", x);
    }

    Ok(())
}
