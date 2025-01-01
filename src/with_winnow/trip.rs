use std::collections::HashMap;
use winnow::ascii::{alphanumeric1, digit1, float, multispace0, multispace1, space0};
use winnow::combinator::{preceded, repeat, separated_pair, seq, terminated};
use winnow::token::take_until;
use winnow::{PResult, Parser};

#[derive(Debug, PartialEq)]
struct Coordinate {
    lat: f64,
    lon: f64,
}

#[derive(Debug, PartialEq)]
struct Destination {
    name: String,
    coordinate: Coordinate,
    tickets: u32,
}

#[derive(Debug, PartialEq)]
pub struct Itinerary {
    countries: HashMap<String, Vec<Destination>>,
}

fn parse_float<'s>(input: &mut &'s str) -> PResult<f64> {
    float.parse_next(input)
}

fn parse_tickets<'s>(input: &mut &'s str) -> PResult<u32> {
    preceded(space0, digit1.try_map(str::parse)).parse_next(input)
}

fn parse_coordinate<'a>(input: &mut &'a str) -> PResult<Coordinate> {
    preceded(
        space0,
        separated_pair(parse_float, ',', parse_float).map(|(lat, lon)| Coordinate { lat, lon }),
    )
    .parse_next(input)
}

fn parse_country_name<'a>(input: &mut &'a str) -> PResult<&'a str> {
    terminated(alphanumeric1, multispace1).parse_next(input)
}

fn parse_city_name<'a>(input: &mut &'a str) -> PResult<&'a str> {
    preceded(space0, take_until(1.., ':').map(|v: &str| v.trim())).parse_next(input)
}

fn parse_destination<'a>(input: &mut &'a str) -> PResult<Destination> {
    seq!(
        Destination {
            name: parse_city_name.map(|x| x.to_string()),
            _: (space0, ':', space0),
            coordinate: parse_coordinate,
            _: (space0, ':', space0),
            tickets: parse_tickets,
            _: multispace0
        }
    )
    .parse_next(input)
}

fn parse_destinations<'a>(input: &mut &'a str) -> PResult<(String, Vec<Destination>)> {
    let country_name = parse_country_name
        .map(|x: &str| x.to_string())
        .parse_next(input)?;

    let destinations = repeat(1.., parse_destination).parse_next(input)?;
    Ok((country_name, destinations))
}

pub fn parse_itinerary<'a>(input: &mut &'a str) -> PResult<Itinerary> {
    let mut parse_countries = repeat::<_, _, Vec<_>, _, _>(1.., parse_destinations);
    let countries: HashMap<String, Vec<Destination>> =
        parse_countries.parse_next(input)?.into_iter().collect();

    Ok(Itinerary { countries })
}

#[test]
fn test_parse_country_name() {
    let mut input = "Russia \n";
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

    let (country, destinations) = parse_destinations(&mut input).unwrap();
    println!("{:#?}", destinations);
    assert_eq!(country, "Norway");
    assert_eq!(destinations.len(), 2);
    assert_eq!(destinations[0].name, "Oslo");
    assert_eq!(destinations[1].name, "Bergen");
}

#[test]
fn test_parse_itinerary() {
    let mut input = r#"Russia
    Vladivostok : 43.131621,131.923828 : 4
    Ulan Ude : 51.841624,107.608101 : 2
Norway
    Oslo : 59.914289,10.738739 : 2
    Bergen : 60.388533,5.331856 : 4"#;

    let itinerary = parse_itinerary(&mut input).unwrap();
    println!("{:#?}", itinerary);
    assert_eq!(itinerary.countries.len(), 1);
    assert!(itinerary.countries.contains_key("Russia"));
    assert!(itinerary.countries.contains_key("Norway"));
    assert_eq!(itinerary.countries["Russia"].len(), 2);
    assert_eq!(itinerary.countries["Norway"].len(), 1);
}
