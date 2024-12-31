use std::collections::HashMap;
use winnow::ascii::{digit1, float, space0, space1};
use winnow::combinator::{cut_err, preceded, repeat, separated_pair};
use winnow::error::{ContextError, InputError, StrContext, StrContextValue};
use winnow::token::{take_until, take_while};
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

fn parse_coordinate<'a>(input: &mut &'a str) -> PResult<Coordinate> {
    preceded(
        (':', space0),
        separated_pair(parse_float, ',', parse_float).map(|(lat, lon)| Coordinate { lat, lon }),
    )
    .parse_next(input)
}

fn parse_city_name<'a>(input: &mut &'a str) -> PResult<&'a str> {
    preceded(
        space0,
        cut_err(take_until(1.., ':'))
            .context(StrContext::Expected(StrContextValue::StringLiteral(":")))
            .map(|v: &str| v.trim()),
    )
    .parse_next(input)
}

fn parse_destination<'a>(input: &mut &'a str) -> PResult<Destination> {
    let city_name = parse_city_name.parse_next(input)?;
    // 解析坐标
    let coordinate = parse_coordinate.parse_next(input)?;

    // 跳过冒号和空白
    take_while(1.., |c: char| c == ':' || c.is_whitespace()).parse_next(input)?;

    // 解析票数
    let tickets = digit1.map(|s: &str| s.parse().unwrap()).parse_next(input)?;

    // 跳过尾随的空白和换行符
    take_while(0.., |c: char| c.is_whitespace() || c == '\n').parse_next(input)?;

    Ok(Destination {
        name: city_name.to_string(),
        coordinate,
        tickets,
    })
}

fn parse_country<'a>(input: &mut &'a str) -> PResult<(String, Vec<Destination>)> {
    let country_name = take_while(1.., |c: char| c != '\n')
        .parse_next(input)?
        .trim()
        .to_string();
    // 确保消费掉国家名称后的换行符
    take_while(0.., |c: char| c.is_whitespace()).parse_next(input)?;

    let cities = repeat(1.., parse_destination).parse_next(input)?;

    Ok((country_name, cities))
}

pub fn parse_itinerary<'a>(input: &mut &'a str) -> PResult<Itinerary> {
    let mut parse_countries = repeat::<_, _, Vec<_>, _, _>(1.., parse_country);
    let countries: HashMap<String, Vec<Destination>> =
        parse_countries.parse_next(input)?.into_iter().collect();

    Ok(Itinerary { countries })
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
    Bergen : 60.388533,5.331856 : 4"#;

    let itinerary = parse_itinerary(&mut input).unwrap();
    assert_eq!(itinerary.countries.len(), 1);
    assert!(itinerary.countries.contains_key("Russia"));
    assert!(itinerary.countries.contains_key("Norway"));
    assert_eq!(itinerary.countries["Russia"].len(), 2);
    assert_eq!(itinerary.countries["Norway"].len(), 1);
}
