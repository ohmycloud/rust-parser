use std::collections::HashMap;
use winnow::ascii::{alphanumeric1, digit1, float, newline, space0, space1};
use winnow::combinator::{opt, preceded, repeat, separated_pair, seq, terminated};
use winnow::token::take_until;
use winnow::{ModalResult, Parser};

#[derive(Debug, PartialEq)]
pub struct Coordinate {
    pub lat: f64,
    pub lon: f64,
}

#[derive(Debug, PartialEq)]
pub struct Destination {
    pub name: String,
    pub coordinate: Coordinate,
    pub tickets: u32,
}

fn parse_float<'s>(input: &mut &'s str) -> ModalResult<f64> {
    float.parse_next(input)
}

fn parse_tickets<'s>(input: &mut &'s str) -> ModalResult<u32> {
    preceded(space0, digit1.try_map(str::parse)).parse_next(input)
}

fn parse_coordinate<'a>(input: &mut &'a str) -> ModalResult<Coordinate> {
    preceded(
        space0,
        separated_pair(parse_float, ',', parse_float).map(|(lat, lon)| Coordinate { lat, lon }),
    )
    .parse_next(input)
}

fn parse_country_name<'a>(input: &mut &'a str) -> ModalResult<&'a str> {
    terminated(alphanumeric1, newline).parse_next(input)
}

fn parse_city_name<'a>(input: &mut &'a str) -> ModalResult<&'a str> {
    preceded(space1, take_until(1.., ':').map(|v: &str| v.trim())).parse_next(input)
}

fn parse_destination<'a>(input: &mut &'a str) -> ModalResult<Destination> {
    seq!(
        Destination {
            name: parse_city_name.map(|x| x.to_string()),
            _: (space0, ':', space0),
            coordinate: parse_coordinate,
            _: (space0, ':', space0),
            tickets: parse_tickets,
            _: opt(newline)
        }
    )
    .parse_next(input)
}

fn parse_destinations<'a>(input: &mut &'a str) -> ModalResult<HashMap<&'a str, Vec<Destination>>> {
    let country_name = parse_country_name.parse_next(input)?;

    let destinations = repeat(1.., parse_destination).parse_next(input)?;
    Ok(std::iter::once((country_name, destinations)).collect())
}

pub fn parse_trips<'a>(
    input: &mut &'a str,
) -> ModalResult<Vec<HashMap<&'a str, Vec<Destination>>>> {
    repeat(0.., parse_destinations).parse_next(input)
}

#[test]
fn test_parse_country_name() {
    let mut input = "Russia\n";
    let country_name = parse_country_name(&mut input);
    assert_eq!(country_name, Ok("Russia"))
}

#[test]
fn test_parse_coordinate() {
    let mut input = "59.914289,10.738739";
    assert_eq!(
        parse_coordinate(&mut input),
        Ok(Coordinate {
            lat: 59.914289,
            lon: 10.738739
        })
    );
}

#[test]
fn test_parse_destination() {
    let mut input = " Oslo : 59.914289,10.738739 : 2";
    assert_eq!(
        parse_destination(&mut input),
        Ok(Destination {
            name: "Oslo".to_string(),
            coordinate: Coordinate {
                lat: 59.914289,
                lon: 10.738739
            },
            tickets: 2,
        })
    );
}

#[test]
fn test_parse_destinations() {
    let mut input = r#"Norway
    Oslo : 59.914289,10.738739 : 2
    Bergen : 60.388533,5.331856 : 4"#;

    let trips = parse_trips(&mut input).unwrap();
    let mut trip: HashMap<&str, Vec<Destination>> = HashMap::new();
    trip.insert(
        "Norway",
        vec![
            Destination {
                name: "Oslo".to_string(),
                coordinate: Coordinate {
                    lat: 59.914289,
                    lon: 10.738739,
                },
                tickets: 2,
            },
            Destination {
                name: "Bergen".to_string(),
                coordinate: Coordinate {
                    lat: 60.388533,
                    lon: 5.331856,
                },
                tickets: 4,
            },
        ],
    );

    assert_eq!(trips, vec![trip]);
}

#[test]
fn test_parse_itinerary() {
    let mut input = r#"Russia
    Vladivostok : 43.131621,131.923828 : 4
    Ulan Ude : 51.841624,107.608101 : 2
Norway
    Oslo : 59.914289,10.738739 : 2
    Bergen : 60.388533,5.331856 : 4"#;

    let trips = parse_trips(&mut input).unwrap();
    println!("{:#?}", trips);
}
