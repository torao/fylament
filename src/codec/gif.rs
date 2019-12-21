extern crate gif;

use std::fs::File;

use gif::SetParameter;

use crate::codec::{Codec, Decoder, Encoder};
use crate::error::FylmError;
use crate::image::Image;

#[derive(Debug)]
struct GIFCodec;

impl Codec for GIFCodec {
  fn new_decoder(&self) -> &dyn Decoder {
    &GIFDecoder {}
  }

  fn new_encoder(&self) -> &dyn Encoder {
    &GIFEncoder {}
  }
}

#[derive(Debug)]
struct GIFDecoder;

impl Decoder for GIFDecoder {
  fn read(&self, file: &mut File) -> Result<&dyn Image, FylmError> {
    let mut decoder = gif::Decoder::new(file);
    decoder.set(gif::ColorOutput::RGBA);
    let mut decoder = decoder.read_info()?;
    unimplemented!()
  }
}

#[derive(Debug)]
struct GIFEncoder;

impl Encoder for GIFEncoder {
  fn write(&self, image: &dyn Image, file: &mut File) -> Result<u64, FylmError> {
    unimplemented!()
  }
}
