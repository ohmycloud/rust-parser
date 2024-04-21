#!warn(dead_code)]
#![warn(unused_variables)]

use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{alpha1, space0, space1, u16};
use nom::IResult;
use nom::sequence::{tuple,separated_pair};
use nom::multi::many1;
use nom::number::complete::double;

#[derive(Debug, PartialEq)]
struct Coordinate {
    lat: f64,
    lon: f64,
}

#[derive(Debug, PartialEq)]
struct Destination {
    name: String,
    coord: Coordinate,
    sales: u16
}

#[derive(Debug, PartialEq)]
struct Trip {
    country: String,
    destination: Vec<Destination>
}

fn parse_country(input: &str) -> IResult<&str, &str> {
    alpha1(input)
}

fn parse_space(input: &str) -> IResult<&str, &str> {
    space1(input)
}

fn parse_space0(input: &str) -> IResult<&str, &str> {
    space0(input)
}

fn parse_unwanted(input: &str) -> IResult<&str, (&str, &str, &str)> {
    let mut parser = tuple((parse_space0, tag(":"), parse_space0));
    parser(input)
}

fn parse_destination_name(input: &str) -> IResult<&str, &str> {
    let (input, name) = take_until(" : ")(input)?;
    Ok((input, name))
}

fn parse_lat_long(input: &str) -> IResult<&str, Coordinate> {
    let mut parser = separated_pair(double, tag(","), double);
    let (input, (lat, lon)) = parser(input)?;
    Ok((input,Coordinate {lat, lon} ))
}

fn parse_destination(input: &str) -> IResult<&str, Destination> {
    let mut parser = tuple((space1, parse_destination_name, tag(" : "), parse_lat_long, tag(" : "), u16));
    let (input, (_, name, t, coord, _, sales)) = parser(input)?;
    Ok((input, Destination { name: name.into(), coord, sales}))
}

fn parse_trip(input: &str) -> IResult<&str, Trip> {
    let mut parser = tuple((parse_country, space0, many1(tuple((parse_destination, space0)))));
    let (input, (country, _, destination)) = parser(input)?;
    Ok((input, Trip {
        country: country.into(),
        destination: destination.into_iter().map(|(d, _)| d).collect()
    }))
}

fn parse_multi_trip(input: &str) -> IResult<&str, Vec<Trip>> {
    let mut parser = many1(parse_trip);
    let (input, trips) = parser(input)?;
    Ok((input, trips))
}

fn main() {
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
    Bern : 46.949076,7.448151 : 1
    "#;

    println!("{:?}", parse_multi_trip(input));
}

#[test]
fn test_destination() {
    let input = "    Oslo : 59.914289,10.738739 : 2";
    assert_eq!(parse_destination(input), Ok(("", Destination {
        name: "Oslo".into(),
        coord: Coordinate {lat: 59.914289, lon: 10.738739},
        sales: 2
    })));
}

#[test]
fn test_destination_name() {
    let input = "Oslo : 59.914289,10.738739 : 2";
    assert_eq!(parse_destination_name(input), Ok((" : 59.914289,10.738739 : 2", "Oslo".into())));
}

#[test]
fn test_lat_lon() {
    let input = "59.914289,10.738739";
    assert_eq!(parse_lat_long(input), Ok(("", Coordinate {lat: 59.914289, lon: 10.738739})));
}

#[test]
fn test_parse_trip() {
    let input = r#"Russia
    Vladivostok : 43.131621,131.923828 : 4
    Ulan Ude : 51.841624,107.608101 : 2
    Saint Petersburg : 59.939977,30.315785 : 10
    "#;

    assert_eq!(parse_trip(input), Ok(("", Trip {
        country: "Russia".into(),
        destination: vec![
            Destination {
                name: "Vladivostok".into(),
                coord: Coordinate { lat: 43.131621, lon: 131.923828},
                sales: 4
            },
            Destination {
                name: "Ulan Ude".into(),
                coord: Coordinate { lat: 51.841624, lon: 107.608101},
                sales: 2
            },
            Destination {
                name: "Saint Petersburg".into(),
                coord: Coordinate { lat: 59.939977, lon: 30.315785},
                sales: 10
            },
        ]
    })));
}