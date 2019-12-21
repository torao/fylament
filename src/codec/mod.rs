use std::fs::File;

use super::error::FylmError;
use super::image::RasterImage;

pub trait Codec {
  fn new_decoder(&self) -> &dyn Decoder;
  fn new_encoder(&self) -> &dyn Encoder;
}

pub trait Decoder {
  /// Read image from specified file.
  fn read_from_file(&self, file_name: &String) -> Result<&RasterImage, FylmError> {
    let mut file = File::open(file_name)?;
    self.read(&mut file)
  }

  /// Read image from specified file.
  fn read(&self, file: &mut File) -> Result<&RasterImage, FylmError>;
}

pub trait Encoder {
  fn write_to_file(&self, image: &RasterImage, file_name: &String) -> Result<u64, FylmError> {
    let mut file = File::open(file_name)?;
    self.write(image, &mut file)
  }
  fn write(&self, image: &RasterImage, file: &mut File) -> Result<u64, FylmError>;
}

pub mod gif;
