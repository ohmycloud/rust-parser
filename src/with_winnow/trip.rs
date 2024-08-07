use winnow::ascii::{digit1, float};
use winnow::combinator::{delimited, repeat, separated, separated_pair};
use winnow::error::InputError;
use winnow::{PResult, Parser};
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct Coordinate {
    lat: f64,
    lon: f64,
}

#[derive(Debug, PartialEq)]
struct City {
    name: String,
    coordinate: Coordinate,
    tickets: u32,
}

#[derive(Debug, PartialEq)]
struct Itinerary {
    countries: HashMap<String, Vec<City>>,
}

fn parse_float<'s>(input: &mut &'s str) -> PResult<f64, InputError<&'s str>> {
    float.parse_next(input)
}

fn parse_coordinate<'a>(input: &mut &'a str) -> PResult<Coordinate, InputError<&'a str>> {
    separated_pair(parse_float, ',', parse_float)
        .map(|(lat, lon)| Coordinate { lat, lon })
        .parse_next(input)
}

fn parse_city<'a>(input: &mut &'a str) -> PResult<City, InputError<&'a str>> {
    let (name, coordinate, tickets) = (
        delimited(
            winnow::token::take_while(0.., |c: char| c.is_whitespace()),
            winnow::token::take_while(1.., |c: char| c != ':'),
            ':',
        ),
        delimited(
            winnow::token::take_while(0.., |c: char| c.is_whitespace()),
            parse_coordinate,
            ':',
        ),
        delimited(
            winnow::token::take_while(0.., |c: char| c.is_whitespace()),
            digit1.map(|s: &str| s.parse().unwrap()),
            winnow::token::take_while(0.., |c: char| c.is_whitespace() || c == '\n'),
        ),
    ).parse_next(input)?;

    Ok(City {
        name: name.trim().to_string(),
        coordinate,
        tickets,
    })
}

fn parse_country<'a>(input: &mut &'a str) -> PResult<(String, Vec<City>), InputError<&'a str>> {
    let country_name = winnow::token::take_while(1.., |c: char| c != '\n')
        .parse_next(input)?
        .trim()
        .to_string();
    input.strip_prefix('\n').unwrap();

    let cities = winnow::combinator::repeat(1.., parse_city)
        .parse_next(input)?;

    Ok((country_name, cities))
}

pub fn parse_itinerary<'a>(input: &mut &'a str) -> PResult<Itinerary, InputError<&'a str>> {
    let mut parse_countries = repeat(1.., parse_country);
    let countries: HashMap<String, Vec<City>> = parse_countries
        .parse_next(input)?
        .into_iter()
        .collect();

    Ok(Itinerary { countries })
}

#[test]
fn test_parse_city() {
    let mut input = "Oslo : 59.914289,10.738739 : 2\n";
    assert_eq!(
        parse_city(&mut input),
        Ok(City {
            name: "Oslo".to_string(),
            coordinate: Coordinate { lat: 59.914289, lon: 10.738739 },
            tickets: 2,
        })
    );
}

#[test]
fn test_parse_country() {
    let mut input = r#"Norway
    Oslo : 59.914289,10.738739 : 2
    Bergen : 60.388533,5.331856 : 4
"#;
    let (country, cities) = parse_country(&mut input).unwrap();
    assert_eq!(country, "Norway");
    assert_eq!(cities.len(), 2);
    assert_eq!(cities[0].name, "Oslo");
    assert_eq!(cities[1].name, "Bergen");
}

#[test]
fn test_parse_itinerary() {
    let mut input = r#"Russia
    Vladivostok : 43.131621,131.923828 : 4
    Ulan Ude : 51.841624,107.608101 : 2
Norway
    Oslo : 59.914289,10.738739 : 2
"#;
    let itinerary = parse_itinerary(&mut input).unwrap();
    assert_eq!(itinerary.countries.len(), 2);
    assert!(itinerary.countries.contains_key("Russia"));
    assert!(itinerary.countries.contains_key("Norway"));
    assert_eq!(itinerary.countries["Russia"].len(), 2);
    assert_eq!(itinerary.countries["Norway"].len(), 1);
}