use winnow::ascii::space1;
use winnow::prelude::*;
use winnow::Parser;
use winnow::ascii::digit1;
use winnow::token::one_of;
use winnow::combinator::separated;
use winnow::combinator::seq;
use winnow::combinator::preceded;
use winnow::ascii::till_line_ending;
use winnow::token::take_until;

#[derive(Debug)]
pub struct Socket<'a> {
    pub ip_address: &'a str,
    pub port: u16,
}

#[derive(Debug)]
pub struct IotLog<'a> {
    pub data_time: &'a str,
    pub client_socket: Socket<'a>,
    pub server_socket: Socket<'a>,
    pub payload: &'a str
}

pub fn parse_datetime<'s>(s: &mut &'s str) -> PResult<&'s str> {
    let mut parsed = separated(1.., digit1, one_of(['-', ':', '.', ' ']))
        .map(|()|())
        .take();

    parsed.parse_next(s)
}


fn parse_socket<'a>(s: &mut &'a str) -> PResult<Socket<'a>> {
    // 解析 IP 地址
    let mut parsed_ip = separated(1.., digit1, '.')
        .map(|()|())
        .take();
    // 解析端口号
    let parsed_port = digit1.map(|port: &str| port.parse::<u16>().unwrap());

    seq!(Socket {
        ip_address: parsed_ip,
        _: ':',
        port: parsed_port,
    }).parse_next(s)
}

// 解析 payload 荷载
fn parse_payload<'a>(s: &mut &'a str) -> PResult<&'a str> {
    let parsed_payload = preceded(':', till_line_ending);
    let mut parsed_payload = preceded(take_until(1.., ":"), parsed_payload);

    parsed_payload.parse_next(s)
}


pub fn parse_log<'a>(s: &mut &'a str) -> PResult<IotLog<'a>> {
    seq!(
        IotLog {
            data_time: parse_datetime,
            _: space1,
            _: '[',
            client_socket: parse_socket,
            _: '#',
            server_socket: parse_socket,
            _: ']',
            payload: parse_payload
        }
    ).parse_next(s)
}
