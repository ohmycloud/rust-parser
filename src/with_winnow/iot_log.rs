use winnow::combinator::delimited;
use winnow::prelude::*;
use winnow::Parser;
use winnow::ascii::digit1;
use winnow::token::one_of;
use winnow::combinator::separated;


pub fn parse_datetime<'s>(s: &mut &'s str) -> PResult<&'s str> {
    let mut parsed = separated(1.., digit1, one_of(['-', ':', '.', ' ']))
        .map(|()|())
        .take();

    parsed.parse_next(s)
}

fn parse_ip_pair<'s>(s: &mut &'s str) -> PResult<&'s str> {
    let mut parsed = separated(1.., digit1, one_of(['.', ':']))
        .map(|()|())
        .take();

    parsed.parse_next(s)
}

fn parse_channel_info<'s>(s: &mut &'s str) -> PResult<&'s str> {
    let parsed = separated(1.., parse_ip_pair, '#')
        .map(|()|())
        .take();

    let mut parsed = delimited('[', parsed, ']').take();

    parsed.parse_next(s)
}