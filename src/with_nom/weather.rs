use std::collections::HashMap;
use nom::character::complete::{alpha1, newline, none_of, space0, space1, u16};
use nom::IResult;
use nom::bytes::complete::tag;
use nom::multi::{many0, many1};
use nom::number::complete::double;
use nom::sequence::tuple;

// 解析 `Name= Jan Mayen` 字符串中 `=` 号左边的字符串
fn parse_key(input: &str) -> IResult<&str, String> {
    let (input, key) = many1(none_of("="))(input)?;
    let key = key.into_iter().collect::<String>();
    Ok((input, key))
}


// 解析 `Name= Jan Mayen` 字符串中 `=` 号右边的字符串
fn parse_value(input: &str) -> IResult<&str, String> {
    let (input, value) = many1(none_of("\n"))(input)?;
    let value = value.into_iter().collect::<String>();
    Ok((input, value))
}

fn parse_temperature(input: &str) -> IResult<&str, Vec<f64>> {
    let mut parser = many1(tuple((space1, double)));
    let (input, temps) = parser(input)?;
    let temps = temps.into_iter().map(|x| x.1).collect();
    Ok((input, temps))
}

fn parse_keyval(input: &str) -> IResult<&str, KeyVal> {
    let mut parser = tuple((many0(newline), space1, parse_key, tag("="), space0, parse_value, many0(newline)));
    let (input, (_, _, key, _, _, value, _)) = parser(input)?;
    Ok((input, KeyVal { key, value }))
}

fn parse_keyvals(input: &str) -> IResult<&str, Vec<KeyVal>> {
    let (input, keyvals) = many1(parse_keyval)(input)?;
    Ok((input, keyvals))
}

fn parse_observation(input: &str) -> IResult<&str, HashMap<String, Vec<f64>>> {
    let mut parser = tuple((many0(newline), space1, u16, parse_temperature, many0(newline)));
    let (input, (_, _, year, temps, _)) = parser(input)?;
    let mut hash: HashMap<String, Vec<f64>> = HashMap::new();
    hash.insert(format!("{}", year), temps);
    Ok((input, hash))
}

fn parse_observations(input: &str) -> IResult<&str, Vec<HashMap<String, Vec<f64>>>> {
    let mut parser = tuple((many0(newline), space1, alpha1, tag(":"), many0(newline), many1(parse_observation)));
    let (input, (_, _, _, _, _, obs)) = parser(input)?;
    Ok((input, obs))
}

pub fn parse_weather(input: &str) -> IResult<&str, (Vec<KeyVal>, Vec<HashMap<String, Vec<f64>>>)> {
    let mut parser = tuple((parse_keyvals, parse_observations));
    let (input, res) = parser(input)?;
    Ok((input, res))
}

#[derive(Debug, PartialEq)]
pub struct WeatherStation {
    name: String,
    country: String,
    observations: HashMap<String, Vec<f32>>
}

#[derive(Debug, PartialEq, Eq)]
pub struct KeyVal {
    key: String,
    value: String,
}

#[test]
fn test_keyval() {
    let input = "    Name= Jan Mayen";
    assert_eq!(parse_keyval(input), Ok(("", KeyVal {
        key: "Name".into(),
        value: "Jan Mayen".into(),
    })));
}

#[test]
fn test_temperature() {
    let input = " -4.4 -7.1 -6.8 -4.3 -0.8  2.2  4.7  5.8  2.7 -2.0 -2.1 -4.0";
    assert_eq!(parse_temperature(input), Ok(("", vec![
        -4.4, -7.1, -6.8, -4.3, -0.8,  2.2,  4.7,  5.8,  2.7, -2.0, -2.1, -4.0
    ])));
}

#[test]
fn test_observation() {
    let input = "    1921 -4.4 -7.1 -6.8 -4.3 -0.8  2.2  4.7  5.8  2.7 -2.0 -2.1 -4.0";
    let mut hashmap: HashMap<String, Vec<f64>> = HashMap::new();
    hashmap.insert("1921".into(), vec![-4.4, -7.1, -6.8, -4.3, -0.8,  2.2,  4.7,  5.8,  2.7, -2.0, -2.1, -4.0]);
    assert_eq!(parse_observation(input), Ok(("", hashmap)));
}

#[test]
fn test_keyvals() {
    let input = r#"
    Name= Jan Mayen
    Country= NORWAY
    Lat=   70.9
    Long=    8.7
    Height= 10
    Start year= 1921
    End year= 2009"#;

    assert_eq!(parse_keyvals(input), Ok(("", vec![
        KeyVal {key: "Name".into(), value: "Jan Mayen".into() },
        KeyVal {key: "Country".into(), value: "NORWAY".into() },
        KeyVal {key: "Lat".into(), value: "70.9".into() },
        KeyVal {key: "Long".into(), value: "8.7".into() },
        KeyVal {key: "Height".into(), value: "10".into() },
        KeyVal {key: "Start year".into(), value: "1921".into() },
        KeyVal {key: "End year".into(), value: "2009".into() },
    ])));
}

#[test]
fn test_observations() {
    let input = r#"
    Obs:
    1921 -4.4 -7.1 -6.8 -4.3 -0.8  2.2  4.7  5.8  2.7 -2.0 -2.1 -4.0
    1922 -0.9 -1.7 -6.2 -3.7 -1.6  2.9  4.8  6.3  2.7 -0.2 -3.8 -2.6
    2008 -2.8 -2.7 -4.6 -1.8  1.1  3.3  6.1  6.9  5.8  1.2 -3.5 -0.8
    2009 -2.3 -5.3 -3.2 -1.6  2.0  2.9  6.7  7.2  3.8  0.6 -0.3 -1.3"#;

    let mut hashmap1921: HashMap<String, Vec<f64>> = HashMap::new();
    let mut hashmap1922: HashMap<String, Vec<f64>> = HashMap::new();
    let mut hashmap2008: HashMap<String, Vec<f64>> = HashMap::new();
    let mut hashmap2009: HashMap<String, Vec<f64>> = HashMap::new();
    hashmap1921.insert("1921".into(), vec![-4.4, -7.1, -6.8, -4.3, -0.8,  2.2,  4.7,  5.8,  2.7, -2.0, -2.1, -4.0]);
    hashmap1922.insert("1922".into(), vec![-0.9, -1.7, -6.2, -3.7, -1.6,  2.9,  4.8,  6.3,  2.7, -0.2, -3.8, -2.6]);
    hashmap2008.insert("2008".into(), vec![-2.8, -2.7, -4.6, -1.8,  1.1,  3.3,  6.1,  6.9,  5.8,  1.2, -3.5, -0.8]);
    hashmap2009.insert("2009".into(), vec![-2.3, -5.3, -3.2, -1.6,  2.0,  2.9,  6.7,  7.2,  3.8,  0.6, -0.3, -1.3]);
    assert_eq!(parse_observations(input), Ok(("", vec![hashmap1921, hashmap1922, hashmap2008, hashmap2009])));
}