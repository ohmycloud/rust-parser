use nom::bytes::complete::tag;
use nom::character::complete::not_line_ending;
use nom::character::complete::{alphanumeric1, digit1, space1};
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::IResult;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use num::{BigInt, Num};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Yjhy {
    da_values: Value,
    ds_name: String,
    err_code: String,
    ied_name: String,
    ts: String,
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

fn parse_json(input: &str) -> IResult<&str, &str> {
    let mut parser = tuple((tag("D:"), not_line_ending));
    let (input, (_, json)) = parser(input)?;

    let json_str= serde_json::from_str::<Vec<Yjhy>>(json);
    if let Ok(json_str) = json_str {
        println!("{:?}", json_str);
    } else {
        println!("parse failed: {}", json);
    }

    Ok((input, json))
}

fn hex_to_binary(hex: &str) -> String {
    let binary_string = if let Ok(hex_value) = BigInt::from_str_radix(hex.trim_start_matches("0x"), 16) {
        println!("{}:{}", hex,hex_value);
        let binary_string = format!("{:0b}", hex_value);
        binary_string
    } else {
        "".into()
    };
    binary_string
}


#[test]
fn test_server_time() {
    let input = "2024-04-06 02:10:07.714";
    assert_eq!(parse_server_time(input), Ok(("", "2024")));
}

#[test]
fn test_topic_name() {
    let input = "[yjhy/GZYJHYEMS001/report/change]";
    assert_eq!(
        parse_topic_name(input),
        Ok(("", vec!["yjhy", "GZYJHYEMS001", "report", "change"]))
    );
}

#[test]
fn test_json_str() {
    let input = r#"D:[{"daValues":{"GZYJHYGW001PCS1Bay01_MC/ZINV1$ST$PwrDrtSt$stVal":"2"},"dsName":"dsDin","errCode":"0","iedName":"GZYJHYGW001PCS1","ts":"2024-04-06 02:10:07"}]"#;
    assert_eq!(
        parse_json(input),
        Ok((
            "",
            r#"[{"daValues":{"GZYJHYGW001PCS1Bay01_MC/ZINV1$ST$PwrDrtSt$stVal":"2"},"dsName":"dsDin","errCode":"0","iedName":"GZYJHYGW001PCS1","ts":"2024-04-06 02:10:07"}]"#
        ))
    );
}

#[test]
fn test_report_change() {
    // 遥信变化上送
    let input = r#"2024-04-06 02:10:07.714  [yjhy/GZYJHYEMS001/report/change]  D:[{"daValues":{"GZYJHYGW001PCS1Bay01_MC/ZINV1$ST$PwrDrtSt$stVal":"2"},"dsName":"dsDin","errCode":"0","iedName":"GZYJHYGW001PCS1","ts":"2024-04-06 02:10:07"}]"#;
}

#[test]
fn test_ai_change() {
    // 遥测变化上送
    let input = r#"2024-04-06 02:10:01.773  [yjhy/GZYJHYEMS001/report/ai/change]  D:[{"daValues":[[74187.40000000001,0,1712340586],[105,0,1712340586]],"dsName":"GZYJHYGW001BMS1LD0/dsAin","errCode":0,"iedName":"GZYJHYGW001BMS1","inclusion":"0x0000000050000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000","ts":"2024-04-06 02:09:46"}]"#;
}

#[test]
fn test_period() {
    // 周期上送(总召)
    let input = r#"2024-04-06 02:13:42.582  [yjhy/GZYJHYEMS001/report/period]  D:[{"daValues":[[25,0,1712340713],[5,0,1712340713],[14,0,1712340713],[5,0,1712340713],[55,0,1712340713],[-10,0,1712340712],[90,0,1712340713],[-20,0,1712340712],[0,0,0]],"dsName":"GZYJHYGW001ADC1LD0/dsDin","errCode":0,"iedName":"GZYJHYGW001ADC1","inclusion":"0xFF8","ts":"2024-04-06 02:11:53"},{"daValues":[[1,0,1712340713],[1,0,1712340713],[1,0,1712340713],[1,0,1712340713],[0,0,0],[0,0,1712340713]],"dsName":"GZYJHYGW001ADC1LD0/dsPara","errCode":0,"iedName":"GZYJHYGW001ADC1","inclusion":"0xFC","ts":"2024-04-06 02:11:53"},{"daValues":[[0,0,1712070712],[0,0,1712340713],[0,0,1712340713],[0,0,1712340713],[0,0,1712340713],[0,0,1712340713],[0,0,1712340713],[0,0,1712340713],[0,0,1712340713],[0,0,1712340713],[0,0,1712340713],[0,0,1712340713],[0,0,1712340713],[0,0,1712340713],[0,0,1712340713],[0,0,1712340713],[0,0,1712340713],[0,0,1712340713],[0,0,1712340713],[0,0,1712340713],[0,0,1712340713],[0,0,1712340713],[0,0,1712340713],[0,0,1712340713],[0,0,1712340713],[0,0,1712340713],[0,0,1712340713],[0,0,1712340713],[0,0,1712340713],[0,0,1712340713],[0,0,1712340713],[0,0,1712340713],[0,0,1712340713],[0,0,1712340713],[0,0,1712340713]],"dsName":"GZYJHYGW001ADC1LD0/dsWarning","errCode":0,"iedName":"GZYJHYGW001ADC1","inclusion":"0xFFFFFFFFE","ts":"2024-04-06 02:11:53"},{"daValues":[[0.45,0,1712340712],[0.96,0,1712340712],[801.9000000000001,0,1712340712],[49.5,0,1712340712],[29.5,0,1712340712],[999,0,1712340712],[999,0,1712340712],[999,0,1712340712],[0,0,0],[3.338,0,1712340712],[24,0,1712340712],[3.331,0,1712340712],[219,0,1712340722],[3.334,0,1712340712],[0,0,0],[29.5,0,1712340712],[1,0,1712340712],[24.5,0,1712340712],[45,0,1712340712],[26.700000000000003,0,1712340712],[46,0,1712340712],[48,0,1712340712],[44,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[198,0,1712340712],[239,0,1712340712],[160,0,1712340712],[190,0,1712340712],[145.9,0,1712340712],[190,0,1712340712],[145.9,0,1712340712],[74190.40000000001,0,1712340788],[74858.8,0,1712340712],[108,0,1712340788],[252,0,1712340712],[62087000,0,1712340712],[58134000,0,1712340712],[3.334,0,1712340712],[3.334,0,1712340712],[3.335,0,1712340712],[3.335,0,1712340712],[3.334,0,1712340712],[3.335,0,1712340712],[3.335,0,1712340712],[3.336,0,1712340712],[3.336,0,1712340712],[3.336,0,1712340712],[3.335,0,1712340712],[3.336,0,1712340712],[3.335,0,1712340712],[3.335,0,1712340712],[3.334,0,1712340712],[3.337,0,1712340712],[3.336,0,1712340712],[3.337,0,1712340712],[3.336,0,1712340712],[3.336,0,1712340712],[3.336,0,1712340712],[3.336,0,1712340712],[3.337,0,1712340712],[3.338,0,1712340712],[3.337,0,1712340712],[3.337,0,1712340712],[3.336,0,1712340712],[3.336,0,1712340712],[3.336,0,1712340712],[3.336,0,1712340712],[3.336,0,1712340712],[3.336,0,1712340712],[3.334,0,1712340712],[3.335,0,1712340712],[3.335,0,1712340712],[3.335,0,1712340712],[3.335,0,1712340712],[3.335,0,1712340712],[3.336,0,1712340712],[3.336,0,1712340712],[3.335,0,1712340712],[3.336,0,1712340712],[3.335,0,1712340712],[3.336,0,1712340712],[3.335,0,1712340712],[3.335,0,1712340712],[3.334,0,1712340712],[3.335,0,1712340712],[3.334,0,1712340712],[3.335,0,1712340712],[3.335,0,1712340712],[3.334,0,1712340712],[3.335,0,1712340712],[3.336,0,1712340712],[3.336,0,1712340712],[3.336,0,1712340712],[3.336,0,1712340712],[3.335,0,1712340712],[3.334,0,1712340712],[3.334,0,1712340712],[3.334,0,1712340712],[3.334,0,1712340712],[3.335,0,1712340712],[3.336,0,1712340712],[3.334,0,1712340712],[3.333,0,1712340712],[3.334,0,1712340712],[3.333,0,1712340712],[3.334,0,1712340712],[3.334,0,1712340712],[3.335,0,1712340712],[3.335,0,1712340712],[3.335,0,1712340712],[3.335,0,1712340712],[3.333,0,1712340712],[3.334,0,1712340712],[3.333,0,1712340712],[3.334,0,1712340712],[3.333,0,1712340712],[3.334,0,1712340712],[3.334,0,1712340712],[3.333,0,1712340712],[3.334,0,1712340712],[3.336,0,1712340712],[3.334,0,1712340712],[3.334,0,1712340712],[3.335,0,1712340712],[3.336,0,1712340712],[3.336,0,1712340712],[3.334,0,1712340712],[3.334,0,1712340712],[3.335,0,1712340712],[3.334,0,1712340712],[3.334,0,1712340712],[3.334,0,1712340712],[3.333,0,1712340712],[3.336,0,1712340712],[3.336,0,1712340712],[3.337,0,1712340712],[3.336,0,1712340712],[3.336,0,1712340712],[3.337,0,1712340712],[3.337,0,1712340712],[3.337,0,1712340712],[3.336,0,1712340712],[3.336,0,1712340712],[3.336,0,1712340712],[3.337,0,1712340712],[3.336,0,1712340712],[3.336,0,1712340712],[3.336,0,1712340712],[3.336,0,1712340712],[3.337,0,1712340712],[3.335,0,1712340712],[3.335,0,1712340712],[3.336,0,1712340712],[3.335,0,1712340712],[3.335,0,1712340712],[3.336,0,1712340712],[3.336,0,1712340712],[3.336,0,1712340712],[3.337,0,1712340712],[3.336,0,1712340712],[3.335,0,1712340712],[3.335,0,1712340712],[3.335,0,1712340712],[3.336,0,1712340712],[3.335,0,1712340712],[3.332,0,1712340712],[3.332,0,1712340712],[3.331,0,1712340712],[3.331,0,1712340712],[3.332,0,1712340712],[3.333,0,1712340712],[3.333,0,1712340712],[3.333,0,1712340712],[3.333,0,1712340712],[3.333,0,1712340712],[3.333,0,1712340712],[3.332,0,1712340712],[3.332,0,1712340712],[3.332,0,1712340712],[3.331,0,1712340712],[3.332,0,1712340712],[3.331,0,1712340712],[3.331,0,1712340712],[3.331,0,1712340712],[3.331,0,1712340712],[3.331,0,1712340712],[3.331,0,1712340712],[3.332,0,1712340712],[3.333,0,1712340712],[3.332,0,1712340712],[3.332,0,1712340712],[3.332,0,1712340712],[3.332,0,1712340712],[3.331,0,1712340712],[3.331,0,1712340712],[3.332,0,1712340712],[3.331,0,1712340712],[3.334,0,1712340712],[3.333,0,1712340712],[3.334,0,1712340712],[3.334,0,1712340712],[3.334,0,1712340712],[3.334,0,1712340712],[3.335,0,1712340712],[3.334,0,1712340712],[3.334,0,1712340712],[3.334,0,1712340712],[3.334,0,1712340712],[3.333,0,1712340712],[3.334,0,1712340712],[3.334,0,1712340712],[3.335,0,1712340712],[3.334,0,1712340712],[3.334,0,1712340712],[3.333,0,1712340712],[3.332,0,1712340712],[3.333,0,1712340712],[3.333,0,1712340712],[3.334,0,1712340712],[3.334,0,1712340712],[3.334,0,1712340712],[3.336,0,1712340712],[3.333,0,1712340712],[3.335,0,1712340712],[3.333,0,1712340712],[3.333,0,1712340712],[3.334,0,1712340712],[3.334,0,1712340712],[3.333,0,1712340712],[3.332,0,1712340712],[3.331,0,1712340712],[3.332,0,1712340712],[3.332,0,1712340712],[3.332,0,1712340712],[3.332,0,1712340712],[3.331,0,1712340712],[3.333,0,1712340712],[3.333,0,1712340712],[3.332,0,1712340712],[3.332,0,1712340712],[3.332,0,1712340712],[3.332,0,1712340712],[3.332,0,1712340712],[3.332,0,1712340712],[3.333,0,1712340712],[3.332,0,1712340712],[3.332,0,1712340712],[3.332,0,1712340712],[3.332,0,1712340712],[3.332,0,1712340712],[3.332,0,1712340712],[3.333,0,1712340712],[3.333,0,1712340712],[3.333,0,1712340712],[3.334,0,1712340712],[3.331,0,1712340712],[3.332,0,1712340712],[3.332,0,1712340712],[3.332,0,1712340712],[3.332,0,1712340712],[3.332,0,1712340712],[3.335,0,1712340712],[3.336,0,1712340712],[3.335,0,1712340712],[3.335,0,1712340712],[3.335,0,1712340712],[3.336,0,1712340712],[3.336,0,1712340712],[3.336,0,1712340712],[3.336,0,1712340712],[3.337,0,1712340712],[3.335,0,1712340712],[3.336,0,1712340712],[3.335,0,1712340712],[3.335,0,1712340712],[3.335,0,1712340712],[3.335,0,1712340712],[29.5,0,1712340712],[29,0,1712340712],[28.5,0,1712340712],[27,0,1712340712],[25,0,1712340712],[27,0,1712340712],[28,0,1712340712],[28.5,0,1712340712],[27,0,1712340712],[27,0,1712340712],[26.5,0,1712340712],[25.5,0,1712340712],[26,0,1712340712],[27.5,0,1712340712],[28.5,0,1712340712],[28.5,0,1712340712],[26.5,0,1712340712],[26.5,0,1712340712],[26,0,1712340712],[24.5,0,1712340712],[25.5,0,1712340712],[26.5,0,1712340712],[27,0,1712340712],[27,0,1712340712],[27,0,1712340712],[26,0,1712340712],[25.5,0,1712340712],[25,0,1712340712],[25.5,0,1712340712],[26.5,0,1712340712],[26.5,0,1712340712],[26.5,0,1712340712],[26.5,0,1712340712],[26,0,1712340712],[25.5,0,1712340712],[24.5,0,1712340712],[24.5,0,1712340712],[25.5,0,1712340712],[26,0,1712340712],[26.5,0,1712340712],[26,0,1712340712],[26,0,1712340712],[25.5,0,1712340712],[24.5,0,1712340712],[24.5,0,1712340712],[26,0,1712340712],[26.5,0,1712340712],[26,0,1712340712],[27,0,1712340712],[27,0,1712340712],[26.5,0,1712340712],[25,0,1712340712],[25.5,0,1712340712],[26,0,1712340712],[26.5,0,1712340712],[27,0,1712340712],[28,0,1712340712],[27.5,0,1712340712],[26.5,0,1712340712],[26,0,1712340712],[26.5,0,1712340712],[27,0,1712340712],[27.5,0,1712340712],[27.5,0,1712340712],[28.5,0,1712340712],[28,0,1712340712],[28,0,1712340712],[27.5,0,1712340712],[27.5,0,1712340712],[28,0,1712340712],[28,0,1712340712],[28,0,1712340712],[27.5,0,1712340712],[27.5,0,1712340712],[27,0,1712340712],[26,0,1712340712],[26.5,0,1712340712],[28,0,1712340712],[28,0,1712340712],[28,0,1712340712],[29,0,1712340712],[28.5,0,1712340712],[28,0,1712340712],[27,0,1712340712],[27,0,1712340712],[28,0,1712340712],[28,0,1712340712],[28,0,1712340712],[28.5,0,1712340712],[29,0,1712340712],[28.5,0,1712340712],[27.5,0,1712340712],[27.5,0,1712340712],[27,0,1712340712],[27,0,1712340712],[27,0,1712340712],[27.5,0,1712340712],[27.5,0,1712340712],[27,0,1712340712],[26,0,1712340712],[26,0,1712340712],[27,0,1712340712],[27.5,0,1712340712],[27,0,1712340712],[27,0,1712340712],[27,0,1712340712],[26,0,1712340712],[25.5,0,1712340712],[26,0,1712340712],[26.5,0,1712340712],[27,0,1712340712],[27,0,1712340712],[26.5,0,1712340712],[26,0,1712340712],[25.5,0,1712340712],[25,0,1712340712],[25,0,1712340712],[26,0,1712340712],[27,0,1712340712],[26.5,0,1712340712],[46,0,1712340712],[47,0,1712340712],[47,0,1712340712],[47,0,1712340712],[47,0,1712340712],[47,0,1712340712],[47,0,1712340712],[47,0,1712340712],[47,0,1712340712],[47,0,1712340712],[47,0,1712340712],[47,0,1712340712],[47,0,1712340712],[47,0,1712340712],[46,0,1712340712],[48,0,1712340712],[47,0,1712340712],[48,0,1712340712],[47,0,1712340712],[47,0,1712340712],[47,0,1712340712],[47,0,1712340712],[48,0,1712340712],[48,0,1712340712],[48,0,1712340712],[48,0,1712340712],[47,0,1712340712],[47,0,1712340712],[47,0,1712340712],[47,0,1712340712],[47,0,1712340712],[47,0,1712340712],[47,0,1712340712],[47,0,1712340712],[47,0,1712340712],[47,0,1712340712],[47,0,1712340712],[47,0,1712340712],[47,0,1712340712],[47,0,1712340712],[47,0,1712340712],[47,0,1712340712],[47,0,1712340712],[47,0,1712340712],[47,0,1712340712],[47,0,1712340712],[46,0,1712340712],[47,0,1712340712],[47,0,1712340712],[47,0,1712340712],[47,0,1712340712],[46,0,1712340712],[47,0,1712340712],[47,0,1712340712],[47,0,1712340712],[47,0,1712340712],[47,0,1712340712],[47,0,1712340712],[46,0,1712340712],[46,0,1712340712],[46,0,1712340712],[47,0,1712340712],[47,0,1712340712],[47,0,1712340712],[46,0,1712340712],[45,0,1712340712],[46,0,1712340712],[45,0,1712340712],[46,0,1712340712],[46,0,1712340712],[47,0,1712340712],[47,0,1712340712],[47,0,1712340712],[47,0,1712340712],[46,0,1712340712],[46,0,1712340712],[46,0,1712340712],[46,0,1712340712],[45,0,1712340712],[46,0,1712340712],[46,0,1712340712],[46,0,1712340712],[47,0,1712340712],[47,0,1712340712],[46,0,1712340712],[46,0,1712340712],[47,0,1712340712],[47,0,1712340712],[47,0,1712340712],[46,0,1712340712],[46,0,1712340712],[47,0,1712340712],[46,0,1712340712],[46,0,1712340712],[46,0,1712340712],[45,0,1712340712],[47,0,1712340712],[47,0,1712340712],[48,0,1712340712],[47,0,1712340712],[47,0,1712340712],[48,0,1712340712],[48,0,1712340712],[48,0,1712340712],[47,0,1712340712],[48,0,1712340712],[47,0,1712340712],[48,0,1712340712],[47,0,1712340712],[47,0,1712340712],[47,0,1712340712],[47,0,1712340712],[48,0,1712340712],[47,0,1712340712],[47,0,1712340712],[47,0,1712340712],[47,0,1712340712],[47,0,1712340712],[47,0,1712340712],[47,0,1712340712],[47,0,1712340712],[48,0,1712340712],[47,0,1712340712],[47,0,1712340712],[47,0,1712340712],[47,0,1712340712],[47,0,1712340712],[47,0,1712340712],[45,0,1712340712],[45,0,1712340712],[45,0,1712340712],[45,0,1712340712],[45,0,1712340712],[45,0,1712340712],[45,0,1712340712],[45,0,1712340712],[45,0,1712340712],[45,0,1712340712],[45,0,1712340712],[45,0,1712340712],[45,0,1712340712],[45,0,1712340712],[45,0,1712340712],[45,0,1712340712],[45,0,1712340712],[45,0,1712340712],[45,0,1712340712],[45,0,1712340712],[45,0,1712340712],[45,0,1712340712],[45,0,1712340712],[45,0,1712340712],[45,0,1712340712],[45,0,1712340712],[45,0,1712340712],[45,0,1712340712],[45,0,1712340712],[45,0,1712340712],[45,0,1712340712],[44,0,1712340712],[46,0,1712340712],[45,0,1712340712],[46,0,1712340712],[46,0,1712340712],[46,0,1712340712],[46,0,1712340712],[47,0,1712340712],[46,0,1712340712],[46,0,1712340712],[46,0,1712340712],[46,0,1712340712],[45,0,1712340712],[46,0,1712340712],[46,0,1712340712],[47,0,1712340712],[46,0,1712340712],[46,0,1712340712],[45,0,1712340712],[45,0,1712340712],[45,0,1712340712],[45,0,1712340712],[46,0,1712340712],[46,0,1712340712],[46,0,1712340712],[47,0,1712340712],[46,0,1712340712],[47,0,1712340712],[45,0,1712340712],[45,0,1712340712],[46,0,1712340712],[46,0,1712340712],[46,0,1712340712],[45,0,1712340712],[45,0,1712340712],[45,0,1712340712],[45,0,1712340712],[45,0,1712340712],[45,0,1712340712],[45,0,1712340712],[45,0,1712340712],[45,0,1712340712],[45,0,1712340712],[45,0,1712340712],[45,0,1712340712],[45,0,1712340712],[45,0,1712340712],[45,0,1712340712],[45,0,1712340712],[45,0,1712340712],[45,0,1712340712],[45,0,1712340712],[45,0,1712340712],[45,0,1712340712],[45,0,1712340712],[45,0,1712340712],[45,0,1712340712],[45,0,1712340712],[46,0,1712340712],[45,0,1712340712],[45,0,1712340712],[45,0,1712340712],[45,0,1712340712],[45,0,1712340712],[45,0,1712340712],[47,0,1712340712],[47,0,1712340712],[47,0,1712340712],[47,0,1712340712],[47,0,1712340712],[47,0,1712340712],[48,0,1712340712],[48,0,1712340712],[47,0,1712340712],[48,0,1712340712],[47,0,1712340712],[47,0,1712340712],[47,0,1712340712],[47,0,1712340712],[47,0,1712340712],[47,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[100,0,1712340712],[196,0,1712340712],[233,0,1712340712],[161,0,1712340712],[227,0,1712340712],[200,0,1712340712],[222,0,1712340712],[211,0,1712340712],[171,0,1712340712],[183,0,1712340712],[182,0,1712340712],[178,0,1712340712],[161,0,1712340712],[202,0,1712340712],[206,0,1712340712],[233,0,1712340712],[237,0,1712340712],[182,0,1712340712],[204,0,1712340712],[194,0,1712340712],[173,0,1712340712],[230,0,1712340712],[198,0,1712340712],[174,0,1712340712],[217,0,1712340712],[171,0,1712340712],[204,0,1712340712],[166,0,1712340712],[224,0,1712340712],[180,0,1712340712],[160,0,1712340712],[184,0,1712340712],[223,0,1712340712],[165,0,1712340712],[202,0,1712340712],[192,0,1712340712],[167,0,1712340712],[189,0,1712340712],[219,0,1712340712],[172,0,1712340712],[235,0,1712340712],[215,0,1712340712],[193,0,1712340712],[197,0,1712340712],[235,0,1712340712],[177,0,1712340712],[167,0,1712340712],[202,0,1712340712],[187,0,1712340712],[203,0,1712340712],[239,0,1712340712],[232,0,1712340712],[165,0,1712340712],[208,0,1712340712],[222,0,1712340712],[194,0,1712340712],[199,0,1712340712],[220,0,1712340712],[234,0,1712340712],[224,0,1712340712],[223,0,1712340712],[169,0,1712340712],[193,0,1712340712],[217,0,1712340712],[165,0,1712340712],[180,0,1712340712],[209,0,1712340712],[173,0,1712340712],[228,0,1712340712],[193,0,1712340712],[221,0,1712340712],[176,0,1712340712],[204,0,1712340712],[176,0,1712340712],[165,0,1712340712],[201,0,1712340712],[238,0,1712340712],[238,0,1712340712],[238,0,1712340712],[162,0,1712340712],[228,0,1712340712],[235,0,1712340712],[194,0,1712340712],[168,0,1712340712],[192,0,1712340712],[170,0,1712340712],[216,0,1712340712],[197,0,1712340712],[216,0,1712340712],[178,0,1712340712],[211,0,1712340712],[187,0,1712340712],[177,0,1712340712],[202,0,1712340712],[161,0,1712340712],[167,0,1712340712],[235,0,1712340712],[230,0,1712340712],[217,0,1712340712],[214,0,1712340712],[213,0,1712340712],[237,0,1712340712],[185,0,1712340712],[185,0,1712340712],[222,0,1712340712],[222,0,1712340712],[170,0,1712340712],[222,0,1712340712],[177,0,1712340712],[206,0,1712340712],[237,0,1712340712],[187,0,1712340712],[221,0,1712340712],[181,0,1712340712],[230,0,1712340712],[177,0,1712340712],[204,0,1712340712],[193,0,1712340712],[236,0,1712340712],[190,0,1712340712],[222,0,1712340712],[233,0,1712340712],[205,0,1712340712],[191,0,1712340712],[201,0,1712340712],[192,0,1712340712],[182,0,1712340712],[192,0,1712340712],[225,0,1712340712],[210,0,1712340712],[185,0,1712340712],[230,0,1712340712],[220,0,1712340712],[211,0,1712340712],[190,0,1712340712],[170,0,1712340712],[214,0,1712340712],[175,0,1712340712],[196,0,1712340712],[178,0,1712340712],[193,0,1712340712],[170,0,1712340712],[182,0,1712340712],[198,0,1712340712],[203,0,1712340712],[171,0,1712340712],[184,0,1712340712],[238,0,1712340712],[209,0,1712340712],[224,0,1712340712],[218,0,1712340712],[186,0,1712340712],[190,0,1712340712],[161,0,1712340712],[205,0,1712340712],[176,0,1712340712],[225,0,1712340712],[163,0,1712340712],[169,0,1712340712],[217,0,1712340712],[179,0,1712340712],[163,0,1712340712],[229,0,1712340712],[187,0,1712340712],[164,0,1712340712],[196,0,1712340712],[188,0,1712340712],[188,0,1712340712],[200,0,1712340712],[165,0,1712340712],[231,0,1712340712],[195,0,1712340712],[168,0,1712340712],[165,0,1712340712],[205,0,1712340712],[206,0,1712340712],[231,0,1712340712],[161,0,1712340712],[212,0,1712340712],[219,0,1712340712],[192,0,1712340712],[235,0,1712340712],[215,0,1712340712],[208,0,1712340712],[189,0,1712340712],[208,0,1712340712],[196,0,1712340712],[214,0,1712340712],[160,0,1712340712],[165,0,1712340712],[236,0,1712340712],[177,0,1712340712],[162,0,1712340712],[201,0,1712340712],[224,0,1712340712],[177,0,1712340712],[181,0,1712340712],[218,0,1712340712],[172,0,1712340712],[225,0,1712340712],[220,0,1712340712],[231,0,1712340712],[177,0,1712340712],[208,0,1712340712],[199,0,1712340712],[176,0,1712340712],[238,0,1712340712],[227,0,1712340712],[215,0,1712340712],[176,0,1712340712],[227,0,1712340712],[196,0,1712340712],[194,0,1712340712],[226,0,1712340712],[238,0,1712340712],[203,0,1712340712],[227,0,1712340712],[203,0,1712340712],[239,0,1712340712],[181,0,1712340712],[212,0,1712340712],[193,0,1712340712],[211,0,1712340712],[171,0,1712340712],[176,0,1712340712],[181,0,1712340712],[203,0,1712340712],[178,0,1712340712],[220,0,1712340712],[171,0,1712340712],[171,0,1712340712],[193,0,1712340712],[167,0,1712340712],[236,0,1712340712],[177,0,1712340712],[196,0,1712340712],[203,0,1712340712],[214,0,1712340712],[168,0,1712340712],[207,0,1712340712],[168,0,1712340712]],"dsName":"GZYJHYGW001BMS1LD0/dsAin","errCode":0,"iedName":"GZYJHYGW001BMS1","inclusion":"0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFE","ts":"2024-04-06 02:13:08"}]"#;
}

#[test]
fn test_inclusion() -> anyhow::Result<()> {
    let input = "0xFF8";
    assert_eq!(hex_to_binary(input), "111111111000");

    let input = "0xFFFFFFFFE";
    assert_eq!(hex_to_binary(input), "111111111111111111111111111111111110");

    let input = "0xFFFFFFFFFFFE";
    assert_eq!(hex_to_binary(input), "111111111111111111111111111111111111111111111110");

    let input = "0xffff8";
    assert_eq!(hex_to_binary(input), "11111111111111111000");

    let input = "0x1FD3C00800000000";
    assert_eq!(hex_to_binary(input), "1111111010011110000000000100000000000000000000000000000000000");

    let input = "0x0000000050000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";
    assert_eq!(hex_to_binary(input), "1010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000");

    let input = "0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFE";
    assert_eq!(hex_to_binary(input), "1111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111110");
    Ok(())
}