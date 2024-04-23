use winnow::ascii::float;
use winnow::combinator::separated;
use winnow::error::InputError;
use winnow::{IResult, PResult, Parser};

#[derive(Debug, PartialEq)]
struct Coordinate {
    lat: f64,
    lon: f64,
}

fn parse_float<'s>(input: &mut &'s str) -> PResult<f64, InputError<&'s str>> {
    float(input)
}

fn parse_lat_long(input: &str) -> IResult<&str, Vec<f64>> {
    separated(1.., parse_float, ",").parse_peek(input)
}

#[test]
fn test_float() {
    let input = "59.914289";
    assert_eq!(parse_float.parse_peek(input), Ok(("", 59.914289)));
}

#[test]
fn test_lat_lon() {
    let mut input = "59.914289,10.738739";
    assert_eq!(
        parse_lat_long(&mut input),
        Ok(("", vec![59.914289, 10.738739]))
    );
}
