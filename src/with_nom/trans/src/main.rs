use trans_parser::parse_transactions;

fn main() {
    let input = r#"
    CREDIT    04062020    PayPal transfer    $4.99
    CREDIT    04032020    Payroll            $69.73
    DEBIT     04022020    ACH transfer       $38.25
    DEBIT     03242020    IRS tax kind       $52249.98
    "#
    .trim_end();

    if let Ok((_, transactions)) = parse_transactions(input) {
        println!("{:#?}", transactions);
    } else {
        print!("parse failed with: {:?}", input);
    }
}
