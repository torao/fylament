extern crate nom;

use nom::branch::alt;
use nom::branch::permutation;
use nom::bytes::complete::escaped_transform;
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::combinator::map;
use nom::combinator::not;
use nom::IResult;
use nom::sequence::delimited;

use self::nom::bytes::complete::take_while_m_n;
use self::nom::character::is_hex_digit;
use self::nom::combinator::{map_res, value};
use self::nom::multi::count;

fn parser(s: &str) -> IResult<&str, &str> {
  permutation((
    multispace0,
    tag("Image"),
    multispace0,
    char('.'),
    multispace0,
    tag("transform"),
    multispace0,
    char('('),
    multispace0,
    char(')'),
    multispace0,
    char('{'),
  ))(s)
}

fn string(s: &str) -> IResult<&str, &str> {
  delimited(
    char('\"'),
    escaped_transform(not(char('\"')), '\\', alt((
      value(char('b'), "\x08"),
      value(char('t'), "\x09"),
      value(char('n'), "\x0a"),
      value(char('f'), "\x0c"),
      value(char('r'), "\x0d"),
      value(char('\"'), "\""),
      value(char('\''), "\'"),
      value(char('\\'), "\\"),
      map_res(
        permutation((char('u'), take_while_m_n(4, 4, is_hex_digit))),
        |(_, code)| code.parse::<char>(16)),
    ))),
    char('\"'),
  )(s)
}
