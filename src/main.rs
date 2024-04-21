mod trip;

use std::error::Error;
use nom::sequence::{delimited, separated_pair, Tuple};
use nom::IResult;
use nom::bytes::complete::{tag, take_while_m_n};
use nom::character::complete::i32;
use nom::combinator::map_res;

#[derive(Debug, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, PartialEq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

pub fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(input, 16)
}

pub fn is_hex_digit(c: char) -> bool {
    c.is_digit(16)
}

pub fn hex_primary(input: &str) -> IResult<&str, u8> {
    map_res(take_while_m_n(2, 2, is_hex_digit), from_hex)(input)
}

pub fn hex_color(input: &str) -> IResult<&str, Color> {
    let (input, _) = tag("#")(input)?;
    let (input, (red,green, blue)) = (hex_primary, hex_primary, hex_primary).parse(input)?;
    Ok((input,Color {red, green, blue}))
}

pub fn parse_integer_pair(input: &str) -> IResult<&str, (i32, i32)> {
    separated_pair(i32, tag(", "), i32)(input)
}

pub fn parse_point(input: &str) -> IResult<&str, Point> {
    let (remaining, (x, y)) = delimited(
        tag("("), 
        parse_integer_pair, 
        tag(")")
    )(input)?;
    Ok((remaining, Point {x, y}))
}

pub fn do_nothing_parser(input: &str) -> IResult<&str, &str> {
    Ok((input, ""))
}

pub fn parse_input(input: &str) -> IResult<&str, &str> {
    tag("abc")(input)
}

fn main() -> Result<(), Box<dyn Error>> {
    let (remaining_input, output) = do_nothing_parser("abcdefg")?;
    assert_eq!(remaining_input, "abcdefg");
    assert_eq!(output, "");

    let (leftover_input, output) = parse_input("abcdefg")?;
    assert_eq!(leftover_input, "defg");
    assert_eq!(output, "abc");
    assert!(parse_input("defdefg").is_err());

    // parse point
    let (_, parsed) = parse_point("(3, 5)")?;
    assert_eq!(parsed, Point {x: 3, y: 5});

    let (_, parsed) = parse_point("(2, -4)")?;
    assert_eq!(parsed, Point {x: 2, y: -4});

    let parsing_error = parse_point("(,3");
    assert!(parsing_error.is_err());

    let parsing_error = parse_point("Ferris");
    assert!(parsing_error.is_err());
    Ok(())
}

#[test]
fn parse_color() {
    assert_eq!(
        hex_color("#2F14DF"),
        Ok(( 
            "",
            Color {
                red: 47,
                green: 20,
                blue: 223,
            }
        ))
    );
}