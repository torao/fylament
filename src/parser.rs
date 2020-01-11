extern crate nom;

use failure::_core::char::{decode_utf16, REPLACEMENT_CHARACTER};
use nom::branch::{alt, permutation};
use nom::bytes::complete::{escaped_transform, tag, take_while_m_n};
use nom::character::complete::{char, multispace0};
use nom::character::is_hex_digit;
use nom::combinator::{map, map_res, not, value};
use nom::IResult;
use nom::multi::count;
use nom::sequence::delimited;

use self::nom::bytes::complete::take_while_m_n;
use self::nom::character::complete::hex_digit1;
use self::nom::character::is_hex_digit;
use self::nom::combinator::{map_res, opt, value};
use self::nom::multi::count;

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

/// Parse string literal that is surrounded by `"` and is able to contain escape sequence.
///
/// | Escape Sequence | Description |
/// |+-----|+-----------------------|
/// | `\b` | backspace |
/// | `\t` | horizontal tab |
/// | `\f` | vertical tab |
/// | `\n` | line feed |
/// | `\r` | carriage return |
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