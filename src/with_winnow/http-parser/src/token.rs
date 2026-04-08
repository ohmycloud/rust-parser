#[derive(Debug, PartialEq)]
pub struct HttpRequest<'a> {
    pub request_line: RequestLine<'a>,
    pub headers: Vec<Header<'a>>,
    pub body: Option<&'a str>,
}

#[derive(Debug, PartialEq)]
pub struct RequestLine<'a> {
    pub method: &'a str,
    pub path: &'a str,
    pub version: &'a str,
}

#[derive(Debug, PartialEq)]
pub struct Header<'a> {
    pub key: &'a str,
    pub value: &'a str,
}
