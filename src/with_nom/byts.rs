use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, digit1, newline, not_line_ending, space1};
use nom::{IResult, Needed};
use nom::multi::{many0, separated_list1};
use nom::sequence::tuple;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Byts {
    pub data: Value,
    pub data_type: u8,
    pub device_key: String,
    pub device_sn: String,
    pub product_key: String,
    pub timestamp: u64
}

// 解析服务器时间
fn parse_server_time(input: &str) -> IResult<&str, &str> {
    let mut parser = tuple((
        digit1,
        tag("-"),
        digit1,
        tag("-"),
        digit1,
        space1,
        digit1,
        tag(":"),
        digit1,
        tag(":"),
        digit1,
        tag("."),
        digit1,
    ));
    let (input, (year, _, month, _, day, _, hour, _, minute, _, seconds, _, micro_sec)) =
        parser(input)?;
    Ok((input, year))
}

fn parse_topic_name(input: &str) -> IResult<&str, Vec<&str>> {
    let mut parser = tuple((tag("["), separated_list1(tag("/"), alphanumeric1), tag("]")));
    let (input, (_, topics, _)) = parser(input)?;
    Ok((input, topics))
}

fn parse_json_str(input: &str) -> IResult<&str, Byts> {
    let mut parser = tuple((tag("D:"), not_line_ending));
    let (input, (_, json)) = parser(input)?;
    let parsed= serde_json::from_str::<Byts>(json);
    if let Ok(parsed) = parsed {
        Ok((input, parsed))
    } else {
        Err(nom::Err::Incomplete(Needed::Unknown))
    }
}

pub fn parse_log(input: &str) -> IResult<&str, Byts> {
    let mut parser = tuple((parse_server_time, space1, parse_topic_name, space1, parse_json_str, many0(newline)));
    let (input, (ts, _, topic_name, _, json_str, _)) = parser(input)?;
    Ok((input, json_str))
}

#[test]
fn test_server_time() {
    let input = "2024-05-05 00:00:03.294";
    assert_eq!(parse_server_time(input), Ok(("", "2024")));
}

#[test]
fn test_topic_name() {
    let input = "[byts/DATA/CN18b08cf88f1]";
    assert_eq!(
        parse_topic_name(input),
        Ok(("", vec!["byts", "DATA", "CN18b08cf88f1"]))
    );
}

#[test]
fn test_json_str() {
    let input = r#"2024-05-05 00:00:03.294  [byts/DATA/CN18b08cf88f1]  D:{"dataType":1,"deviceKey":"CN-18b08cf88f1","deviceSn":"CN-18b08cf88f1","productKey":"8sffV8oVNAd","timestamp":1714838402000}"#;

    if let Ok(res) = parse_json_str(input) {
        println!("{:?}", res.1);
    } else {
        println!("{}", input);
    }
}