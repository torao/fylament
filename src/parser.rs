extern crate nom;

use failure::_core::char::{REPLACEMENT_CHARACTER};
use nom::branch::{alt, permutation};
use nom::bytes::complete::{escaped_transform, tag};
use nom::character::complete::{char, multispace0, hex_digit1};
use nom::combinator::{map_res, not, value};
use nom::IResult;
use nom::sequence::delimited;

#[derive(Debug)]
pub enum ParseError {
  StringLiteralError {
    description: String
  }
}

pub fn parser(s: &str) -> IResult<&str, &str> {
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

/// Parse string literal that is surrounded by `"` and is able to contain escape sequence.
///
/// | Escape Sequence | Description |
/// |+-----|+-----------------------|
/// | `\b` | backspace |
/// | `\t` | horizontal tab |
/// | `\f` | vertical tab |
/// | `\n` | line feed |
/// | `\r` | carriage return |
/// | `\"` | double quote |
/// | `\'` | single quote |
/// | `\\` | slash |
/// | `\u{XXXXXXXX}` | unicode character |
///
fn string_literal(s: &str) -> IResult<&str, String> {
  delimited(
    char('\"'),
    escaped_transform(not(char('\"')), '\\', alt((
      value('\x08', char('b')),
      value('\x09', char('t')),
      value('\x0a', char('n')),
      value('\x0c', char('f')),
      value('\x0d', char('r')),
      value('\"', char('\"')),
      value('\'', char('\'')),
      value('\\', char('\\')),
      map_res(permutation((char('u'), unicode_character_literal)), |(_, r)| r)
    ))),
    char('\"'),
  )(s)
}

/// Parse `{XXXXXXXX}` string sequence and return it as Unicode character. In case it's not defined
/// as Unicode, replace to U+FFFD REPLACEMENT CHARACTER instead.
fn unicode_character_literal(s: &str) -> IResult<&str, char> {
  map_res(
    delimited(char('{'), hex_digit1, char('}')),
    |hex| {
      std::char::from_u32(u32::from_str(hex).unwrap()).unwrap_or(REPLACEMENT_CHARACTER)
    },
  )(s)
}