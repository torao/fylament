use std::fs::File;

use super::error::FylmError;
use super::image::Image;

pub trait Codec {
  fn new_decoder(&self) -> &dyn Decoder;
  fn new_encoder(&self) -> &dyn Encoder;
}

pub trait Decoder {
  /// Read image from specified file.
  fn read_from_file(&self, file_name: &String) -> Result<&dyn Image, FylmError> {
    let mut file = File::open(file_name)?;
    self.read(&mut file)
  }

  /// Read image from specified file.
  fn read(&self, file: &mut File) -> Result<&dyn Image, FylmError>;
}

pub trait Encoder {
  fn write_to_file(&self, image: &dyn Image, file_name: &String) -> Result<u64, FylmError> {
    let mut file = File::open(file_name)?;
    self.write(image, &mut file)
  }
  fn write(&self, image: &dyn Image, file: &mut File) -> Result<u64, FylmError>;
}

pub mod gif;
