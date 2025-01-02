use std::collections::HashMap;
use winnow::ascii::{alphanumeric1, digit1, float, newline, space0, till_line_ending};
use winnow::combinator::{opt, preceded, repeat};
use winnow::combinator::{seq, terminated};
use winnow::token::take_until;
use winnow::PResult;
use winnow::Parser;

#[derive(Debug, Eq, PartialEq)]
pub struct Kv<'a> {
    pub key: &'a str,
    pub value: &'a str,
}

#[derive(Debug, PartialEq)]
pub struct WeatherStation<'a> {
    kvs: Vec<Kv<'a>>,
    observations: Vec<HashMap<&'a str, Vec<f64>>>,
}

fn parse_key<'a>(input: &mut &'a str) -> PResult<&'a str> {
    preceded(space0, take_until(1.., "="))
        .map(|x: &str| x.trim())
        .parse_next(input)
}

fn parse_value<'a>(input: &mut &'a str) -> PResult<&'a str> {
    preceded(space0, till_line_ending)
        .map(|x: &str| x.trim())
        .parse_next(input)
}

fn parse_kv_pair<'a>(input: &mut &'a str) -> PResult<Kv<'a>> {
    seq!(
        Kv {
            key: parse_key,
            _: '=',
            value: parse_value
        }
    )
    .parse_next(input)
}

fn parse_kvs<'a>(input: &mut &'a str) -> PResult<Vec<Kv<'a>>> {
    repeat(0.., parse_kv_pair).parse_next(input)
}

fn parse_temperatures<'a>(input: &mut &'a str) -> PResult<Vec<f64>> {
    repeat(1.., preceded(space0, float.map(|x: f64| x))).parse_next(input)
}

fn parse_year<'a>(input: &mut &'a str) -> PResult<&'a str> {
    preceded(space0, digit1).parse_next(input)
}

pub fn parse_observation<'a>(input: &mut &'a str) -> PResult<HashMap<&'a str, Vec<f64>>> {
    Ok(std::iter::once((
        parse_year.parse_next(input)?,
        terminated(parse_temperatures, opt(newline)).parse_next(input)?,
    ))
    .collect())
}

fn parse_obs<'a>(input: &mut &'a str) -> PResult<&'a str> {
    seq!(preceded(space0, alphanumeric1), ':')
        .take()
        .map(|x: &str| x.trim())
        .parse_next(input)
}

pub fn parse_observations<'a>(input: &mut &'a str) -> PResult<Vec<HashMap<&'a str, Vec<f64>>>> {
    repeat(0.., parse_observation).parse_next(input)
}

pub fn parse_weather<'a>(input: &mut &'a str) -> PResult<WeatherStation<'a>> {
    seq!(
        WeatherStation {
            kvs: parse_kvs,
            _: parse_obs,
            observations: parse_observations
        }
    )
    .parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_kv() {
        let mut input = "    Country = NORWAY";
        let expected = parse_kv_pair(&mut input).unwrap();
        assert_eq!(
            expected,
            Kv {
                key: "Country",
                value: "NORWAY"
            }
        )
    }

    #[test]
    fn test_parse_kvs() {
        let mut input = r#"
    Name= Jan Mayen
    Country = NORWAY
    Lat=   70.9
    Long=    8.7
    Height= 10
    Start year= 1921
    End year= 2009"#;

        let expected = parse_kvs(&mut input).unwrap();
        assert_eq!(
            expected,
            vec![
                Kv {
                    key: "Name",
                    value: "Jan Mayen"
                },
                Kv {
                    key: "Country",
                    value: "NORWAY"
                },
                Kv {
                    key: "Lat",
                    value: "70.9"
                },
                Kv {
                    key: "Long",
                    value: "8.7"
                },
                Kv {
                    key: "Height",
                    value: "10"
                },
                Kv {
                    key: "Start year",
                    value: "1921"
                },
                Kv {
                    key: "End year",
                    value: "2009"
                }
            ]
        )
    }

    #[test]
    fn test_parse_temperatures() {
        let mut input = " -4.4 -7.1 -6.8 -4.3 -0.8  2.2  4.7  5.8  2.7 -2.0 -2.1 -4.0";
        let expected = parse_temperatures(&mut input).unwrap();
        assert_eq!(
            expected,
            vec![-4.4, -7.1, -6.8, -4.3, -0.8, 2.2, 4.7, 5.8, 2.7, -2.0, -2.1, -4.0]
        );
    }

    #[test]
    fn test_parse_obs() {
        let mut input = "    Obs:";
        let expected = parse_obs(&mut input).unwrap();
        assert_eq!("Obs:", expected);
    }

    #[test]
    fn test_parse_observation() {
        let mut input = "   1921 -4.4 -7.1 -6.8 -4.3 -0.8  2.2  4.7  5.8  2.7 -2.0 -2.1 -4.0";
        let expected = parse_observation(&mut input).unwrap();
        let mut actual: HashMap<&str, Vec<f64>> = HashMap::new();
        actual.insert(
            "1921",
            vec![
                -4.4, -7.1, -6.8, -4.3, -0.8, 2.2, 4.7, 5.8, 2.7, -2.0, -2.1, -4.0,
            ],
        );
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_observations() {
        let mut input = r#"1921 -4.4 -7.1 -6.8 -4.3 -0.8  2.2  4.7  5.8  2.7 -2.0 -2.1 -4.0
            2009 -2.3 -5.3 -3.2 -1.6  2.0  2.9  6.7  7.2  3.8  0.6 -0.3 -1.3"#;

        let expected = parse_observations(&mut input).unwrap();
        let mut hash1: HashMap<&str, Vec<f64>> = HashMap::new();
        hash1.insert(
            "1921".into(),
            vec![
                -4.4, -7.1, -6.8, -4.3, -0.8, 2.2, 4.7, 5.8, 2.7, -2.0, -2.1, -4.0,
            ],
        );
        let mut hash2: HashMap<&str, Vec<f64>> = HashMap::new();
        hash2.insert(
            "2009",
            vec![
                -2.3, -5.3, -3.2, -1.6, 2.0, 2.9, 6.7, 7.2, 3.8, 0.6, -0.3, -1.3,
            ],
        );
        assert_eq!(expected, vec![hash1, hash2]);
    }
}
