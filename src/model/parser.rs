extern crate nom;

use failure::_core::char::REPLACEMENT_CHARACTER;
use nom::branch::{alt, permutation};
use nom::bytes::complete::{escaped_transform, tag, take_while_m_n};
use nom::character::complete::{char, line_ending, multispace0, none_of, not_line_ending};
use nom::combinator::{map, value};
use nom::IResult;
use nom::sequence::delimited;

use self::nom::bytes::complete::take_until;
use self::nom::number::complete::recognize_float;

#[derive(Debug)]
pub enum ParseError {
  StringLiteralError {
    description: String
  }
}

pub enum Token {
  BooleanLiteral(bool),
  StringLiteral(String),
  IntLiteral(u64),
  FloatLiteral(f64),
}

pub fn parser(_: &str) -> Result<Vec<Token>, ParseError> {
  Result::Ok(vec![])
}

/*
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
*/

pub fn literal(s: &str) -> IResult<&str, Token> {
  alt((
    map(boolean_literal, |b| Token::BooleanLiteral(b)),
    map(string_literal, |s| Token::StringLiteral(s)),
    map(float_literal, |f| Token::FloatLiteral(f)),
  ))(s)
}

/// Parse boolean literal that has a value "true" or "false".
fn boolean_literal(s: &str) -> IResult<&str, bool> {
  alt((
    value(true, tag("true")),
    value(false, tag("false"))
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
      value('\x09', char('t')),
      value('\x0a', char('n')),
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
    alt((
      take_while_m_n(4, 4, |c: char| c.is_ascii_hexdigit()),
      delimited(
        char('{'),
        delimited(
          multispace0,
          take_while_m_n(1, 32, |c: char| c.is_ascii_hexdigit()),
          multispace0),
        char('}'),
      ),
    )),
    |hex| {
      std::char::from_u32(u32::from_str_radix(hex, 16).unwrap()).unwrap_or(REPLACEMENT_CHARACTER)
    },
  )(s)
}

fn float_literal(s: &str) -> IResult<&str, f64> {
  let (s, value) = recognize_float(s)?;
  Ok((s, value.parse::<f64>().unwrap()))
}

/// Either a line comment or block comment.
pub fn comment(s: &str) -> IResult<&str, &str> {
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
