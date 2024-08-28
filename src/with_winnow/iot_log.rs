use winnow::ascii::digit1;
use winnow::combinator::{separated, repeat};
use winnow::{PResult, Parser};

fn parse_datetime<'a>(input: &mut &'a str) -> PResult<&'a str> {
    let date_separator = |i: &mut &str| {
        ("-" | " " | ":" | ".").value(()).parse_next(i)
    };

    let datetime_parser = repeat(2.., (
        digit1,
        date_separator
    )).parse_to_slice();

    datetime_parser.parse_next(input)
}

fn main() {
    let input = "2024-08-26 00:00:00.720";
    match parse_datetime.parse(input) {
        Ok(datetime) => println!("解析结果: {}", datetime),
        Err(e) => eprintln!("解析错误: {}", e),
    }
}