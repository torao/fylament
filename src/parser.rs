extern crate nom;

use failure::_core::char::REPLACEMENT_CHARACTER;
use nom::{Err, IResult};
use nom::branch::{alt, permutation};
use nom::bytes::complete::{escaped_transform, tag};
use nom::bytes::complete::take_till;
use nom::character::complete::{char, hex_digit1, multispace0, none_of};
use nom::combinator::{map, value};
use nom::error::ErrorKind;
use nom::sequence::delimited;

#[derive(Debug)]
pub enum ParseError {
  IOError {
    description: String
  },
  StringLiteralError {
    description: String
  },
}

pub fn parser(s: &str) -> IResult<&str, &str> {
  let (s, _) = multispace0(s)?;
  let (s, _) = tag("Image")(s)?;
  let (s, _) = multispace0(s)?;
  let (s, _) = char('.')(s)?;
  let (s, _) = multispace0(s)?;
  let (s, _) = tag("transform")(s)?;
  let (s, _) = multispace0(s)?;
  let (s, _) = char('(')(s)?;
  let (s, _) = multispace0(s)?;
  let (s, _) = char(')')(s)?;
  let (s, _) = multispace0(s)?;
  let (s, _) = char('{')(s)?;
  let (s, _) = multispace0(s)?;
  let (s, _) = tag("=>")(s)?;
  let (s, content) = take_till(|ch| ch != '}')(s)?;
  let (s, _) = char('}')(s)?;
  let (s, _) = multispace0(s)?;
  if s.len() == 0 {
    return Err(Err::Error((s, ErrorKind::TooLarge)));
  }
  Ok((s, content))
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
    escaped_transform(none_of("\"\\"), '\\', alt((
      value('\x08', char('b')),
      value('\x09', char('t')),
      value('\x0a', char('n')),
      value('\x0c', char('f')),
      value('\x0d', char('r')),
      value('\"', char('\"')),
      value('\'', char('\'')),
      value('\\', char('\\')),
      map(permutation((char('u'), unicode_character_literal)), |(_, c)| c),
    ))),
    char('\"'),
  )(s)
}

/// Parse `{XXXXXXXX}` string sequence and return it as Unicode character. In case it's not defined
/// as Unicode, replace to U+FFFD REPLACEMENT CHARACTER instead.
fn unicode_character_literal(s: &str) -> IResult<&str, char> {
  map(
    delimited(char('{'), hex_digit1, char('}')),
    |hex| {
      std::char::from_u32(u32::from_str_radix(hex, 16).unwrap()).unwrap_or(REPLACEMENT_CHARACTER)
    },
  )(s)
}