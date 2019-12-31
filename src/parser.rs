// #[macro_use]
extern crate nom;

use nom::{
  complete::tag,
  do_parse,
  IResult,
  named,
  whitespace,
};

use self::nom::sequence::delimited;
use self::nom::multi::separated_list;
use self::nom::character::complete::char;

fn string(i: &str) -> IResult<&str, &str> {
  delimited(
    char('\"'),
    parse_str,
    char('\"'),
  )(i)
}

fn array(i:&str) -> IResult<&str, Vec<&str>> {
  delimited(
    char('['),
    separated_list(char(','), parse_string),
    char(']'),
  )(i)
}

named!(name_parser<(&str,&str,&str)>, do_parse!(
  space? >> tag!("Image") >> space? >> tag!(".") >> space? >> tag!("transform") >> tag!("(") >> space? >>
  duration: map_res!(
    |ch| -> ch.is_alphanumeric(),
    from_utf8
  ) >> space? >> tag!(")") >> space? >> tag!("{") >> space? >>
  local_params: map_res!(
    is_identifier,
    from_utf8
  ) >> space? >> tag!("=>") >> space? >> tag!("uri") >> space? >> tag!(":") >> space? >> tag!("[") >> space? >>
  urls: map_res!(
    |ch| ch != ']',
    from_utf8
  ) >> space? >> tag!("]") >> space? >> tag!("}") >> space?
));

pub fn parse(files: Vec<String>) -> u32 {
  10
}
