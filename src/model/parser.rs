extern crate nom;

use failure::_core::char::{ REPLACEMENT_CHARACTER};
use nom::branch::{alt, permutation};
use nom::bytes::complete::{escaped_transform, tag};
use nom::character::complete::{char, hex_digit1, line_ending, multispace0, none_of, not_line_ending};
use nom::combinator::{map, not, value};
use nom::IResult;
use nom::sequence::delimited;
use self::nom::bytes::complete::take_until;

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
    char('}'),
    multispace0,
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
    escaped_transform(none_of("\"\r\n"), '\\', alt((
      value('\x08', char('b')),
      value('\x09', char('t')),
      value('\x0a', char('n')),
      value('\x0c', char('f')),
      value('\x0d', char('r')),
      value('\"', char('\"')),
      value('\'', char('\'')),
      value('\\', char('\\')),
      map(permutation((char('u'), unicode_character_literal)), |(_, r)| r)
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

/// Comment that is constructed with line or block comment.
fn comment(s: &str) -> IResult<&str, &str> {
  alt((line_comment, block_comment))(s)
}

/// Line comment.
fn line_comment(s: &str) -> IResult<&str, &str> {
  delimited(tag("//"), not_line_ending, line_ending)(s)
}

/// Block comment.
fn block_comment(s: &str) -> IResult<&str, &str> {
  delimited(tag("/*"), take_until("*/"), tag("*/"))(s)
}
