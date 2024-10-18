use winnow::ascii::{alpha1, multispace1, newline, space0, space1, till_line_ending};
use winnow::combinator::{opt, preceded, repeat, seq};
use winnow::token::take_until;
use winnow::{PResult, Parser};

#[derive(Debug, PartialEq)]
pub struct HttpRequest<'a> {
    request_line: RequestLine<'a>,
    headers: Vec<Header<'a>>,
    body: Option<&'a str>,
}

#[derive(Debug, PartialEq)]
pub struct RequestLine<'a> {
    method: &'a str,
    path: &'a str,
    version: &'a str,
}

#[derive(Debug, PartialEq)]
pub struct Header<'a> {
    key: &'a str,
    value: &'a str,
}

fn parse_method<'a>(input: &mut &'a str) -> PResult<&'a str> {
    alpha1.parse_next(input)
}

fn parse_path<'a>(input: &mut &'a str) -> PResult<&'a str> {
    preceded(space1, take_until(1.., " ")).parse_next(input)
}

fn parse_version<'a>(input: &mut &'a str) -> PResult<&'a str> {
    preceded(space1, take_until(1.., "\r\n")).parse_next(input)
}

fn parse_request_line<'a>(input: &mut &'a str) -> PResult<RequestLine<'a>> {
    seq!(RequestLine {
        method: parse_method,
        path: parse_path,
        version: parse_version,
    })
    .parse_next(input)
}

fn parse_header<'a>(input: &mut &'a str) -> PResult<Header<'a>> {
    let mut key = preceded(space0, take_until(1.., ":")).map(|x: &str| x.trim());
    let mut value = preceded(space0, till_line_ending).map(|x: &str| x.trim());

    seq!(
        Header {
            key: key,
            _: ':',
            value: value
        }
    )
    .parse_next(input)
}

fn parse_headers<'a>(input: &mut &'a str) -> PResult<Vec<Header<'a>>> {
    let mut headers = repeat(0.., parse_header);
    headers.parse_next(input)
}

fn parse_body<'a>(input: &mut &'a str) -> PResult<Option<&'a str>> {
    let body = preceded(multispace1, till_line_ending).map(|x: &str| x.trim());
    opt(body).parse_next(input)
}

pub fn parse_http_request<'a>(input: &mut &'a str) -> PResult<HttpRequest<'a>> {
    seq!(HttpRequest {
        request_line: parse_request_line,
        headers: parse_headers,
        body: parse_body,
    })
    .parse_next(input)
}

// 测试代码
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_http_request() -> anyhow::Result<()> {
        let mut request = "GET /greeting HTTP/1.1\r\n\
                      Host: localhost:3000\r\n\
                      User-agent: Curl/7.64.1\r\n\
                      Accept: */*\r\n\
                      \r\n\
                      xxxxxxxxxx";

        let result = parse_http_request(&mut request).unwrap();

        assert_eq!(result.request_line.method, "GET");
        assert_eq!(result.request_line.path, "/greeting");
        assert_eq!(result.request_line.version, "HTTP/1.1");

        assert_eq!(result.headers.len(), 3);
        assert_eq!(result.headers[0].key, "Host");
        assert_eq!(result.headers[0].value, "localhost:3000");

        assert_eq!(result.body, Some("xxxxxxxxxx"));

        Ok(())
    }
}
