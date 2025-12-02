use winnowcurl::curl::parser::{curl_cmd_parse, Curl};

#[test]
fn parse_simple_unquoted_url_and_method() {
    let input = r#"curl https://example.com -X POST"#;
    let result = curl_cmd_parse(input).expect("should parse");
    assert_eq!(result.len(), 2);
    match &result[0] { Curl::URL(_) => {}, _ => panic!("first should be URL") }
    match &result[1] {
        Curl::Method(m) => {
            assert_eq!(m.identifier, "-X");
            assert_eq!(m.data.as_deref(), Some("POST"));
        }
        _ => panic!("second should be Method"),
    }
}

#[test]
fn parse_multiline_with_backslashes_and_mixed_quotes() {
    let input = r#"
        curl 'http://query.sse.com.cn/commonQuery.do?jsonCallBack=jsonpCallback89469743&sqlId=COMMON_SSE_SJ_GPSJ_CJGK_MRGK_C&PRODUCT_CODE=01%2C02%2C03%2C11%2C17&type=inParams&SEARCH_DATE=2024-03-18&_=1710914422498' \
        -H 'Accept: */*' -X 'TEST' \
        -H 'Accept-Language: en-US,en;q=0.9,zh-CN;q=0.8,zh;q=0.7' \
        -H 'Cache-Control: no-cache' \
        -H 'Connection: keep-alive' \
        -d 'data1:90' \
        --data 'data2:90/i9fi0sdfsdfk\\jfhaoe' \
        -H 'Cookie: gdp_user_id=gioenc-c2b256a9%2C5442%2C561b%2C9c02%2C71199e7e89g9; VISITED_MENU=%5B%228312%22%5D' \
        -H 'Pragma: no-cache' \
        -H 'Referer: http://www.sse.com.cn/' \
        -H 'User-Agent: UA' \
        -v
    "#;

    let result = curl_cmd_parse(input).expect("parse complex");
    assert!(result.len() >= 10);

    // Verify contains at least one of each expected type
    assert!(result.iter().any(|c| matches!(c, Curl::URL(_))));
    assert!(result.iter().any(|c| matches!(c, Curl::Method(_))));
    assert!(result.iter().any(|c| matches!(c, Curl::Header(_))));
    assert!(result.iter().any(|c| matches!(c, Curl::Data(_))));
    assert!(result.iter().any(|c| matches!(c, Curl::Flag(_))));
}

#[test]
fn parse_data_equals_and_unquoted_forms() {
    let input = r#"curl 'https://example.com/x' -XPOST --data-raw='{"a":1}' --data-urlencode=a=b --data-binary @file.bin -dname=John"#;
    let result = curl_cmd_parse(input).expect("should parse");
    // URL + Method + 4 Data
    assert_eq!(result.len(), 6);

    match &result[0] { Curl::URL(_) => {}, _ => panic!("first should be URL") }
    match &result[1] {
        Curl::Method(m) => assert_eq!(m.data.as_deref(), Some("POST")),
        _ => panic!("second should be Method"),
    }

    let data_idents: Vec<_> = result
        .iter()
        .filter_map(|c| if let Curl::Data(d) = c { Some(d.identifier.clone()) } else { None })
        .collect();
    assert_eq!(data_idents, vec!["--data-raw", "--data-urlencode", "--data-binary", "-d"]);
}

#[test]
fn parse_multiple_flags_long_and_short() {
    let input = r#"curl 'http://a' -k -L --insecure --retry-all-errors --max-time=5 -o out.txt -x http://proxy --cert cert.pem --key key.pem --cacert=ca.pem --output result.json"#;
    let result = curl_cmd_parse(input).expect("parse flags");
    let flags: Vec<_> = result
        .iter()
        .filter_map(|c| match c {
            Curl::Flag(f) if matches!(f.identifier.as_str(),
                "-k" | "-L" | "--insecure" | "--retry-all-errors" |
                "--max-time" | "-o" | "-x" | "--cert" | "--key" | "--cacert" | "--output"
            ) => Some(f.identifier.clone()),
            _ => None,
        })
        .collect();
    assert!(flags.contains(&"-k".into()));
    assert!(flags.contains(&"-L".into()));
    assert!(flags.contains(&"--insecure".into()));
    assert!(flags.contains(&"--retry-all-errors".into()));
    assert!(flags.contains(&"--max-time".into()));
    assert!(flags.contains(&"-o".into()));
    assert!(flags.contains(&"--output".into()));
    assert!(flags.contains(&"-x".into()));
    assert!(flags.contains(&"--cacert".into()));
    assert!(flags.contains(&"--cert".into()));
    assert!(flags.contains(&"--key".into()));
}

#[test]
fn reject_non_curl_string() {
    let err = curl_cmd_parse("echo curl 'http://example.com'").unwrap_err();
    assert!(err.to_lowercase().contains("does not start with curl"));
}

#[test]
fn parse_minimal_https_no_path() {
    let input = "curl https://example.com";
    let result = curl_cmd_parse(input).expect("should parse");
    assert_eq!(result.len(), 1);
    match &result[0] { Curl::URL(url) => {
        // domain captured in path for this URL model
        assert_eq!(url.path, "example.com");
    }, _ => panic!("first should be URL") }
}
