extern crate nom;

use nom::branch::{alt, permutation};
use nom::bytes::complete::{escaped_transform, tag, take_while_m_n};
use nom::character::complete::{char, multispace0};
use nom::character::is_hex_digit;
use nom::combinator::{map, map_res, not, value};
use nom::IResult;
use nom::multi::count;
use nom::sequence::delimited;

#[derive(Debug, Fail)]
pub enum ParseError {
  #[fail(display = "i/o error: {}", description)]
  StringLiteralError {
    description: String
  }
}

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
      value("\x08", char('b')),
      value("\x09", char('t')),
      value("\x0a", char('n')),
      value("\x0c", char('f')),
      value("\x0d", char('r')),
      value("\"", char('\"')),
      value("\'", char('\'')),
      value("\\", char('\\')),
      map_res(
        permutation((char('u'), take_while_m_n(4, 4, is_hex_digit))),
        |(_, code)| code.parse::<char>(16)),
    ))),
    char('\"'),
  )(s)
}

fn hex_to_string(s:&[u8]) -> Result<String, ParseError> {
  for i in
}
