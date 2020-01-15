extern crate failure;

use std::error::Error;
use std::io::Error as IOError;

use failure::Fail;

#[derive(Debug, Fail)]
pub enum FylmError {
  #[fail(display = "i/o error: {}", description)]
  IOError {
    description: String
  },
  #[fail(display = "gif error: {}", description)]
  GIFError {
    description: String
  },
  #[fail(display = "parse error: {}", description)]
  ParseError {
    description: String
  }
}

impl From<IOError> for FylmError {
  fn from(err: IOError) -> Self {
    FylmError::IOError {
      description: err.description().to_string()
    }
  }
}

impl From<gif::DecodingError> for FylmError {
  fn from(err: gif::DecodingError) -> Self {
    FylmError::GIFError {
      description: err.description().to_string()
    }
  }
}