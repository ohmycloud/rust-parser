use log_parser::parse_log;

fn main() -> Result<(), winnow::error::ParseError<&'static str, &'static str>> {
    let mut input = "2024-08-26 00:00:00.720  [117.132.198.154:3649#172.19.85.132:5003] R:68125e48a4000d0103000300c042009cc4de4200";
    let result = parse_log(&mut input);
    if let Ok(parsed) = result {
        println!("{:#?}", parsed);
    }

    Ok(())
}
