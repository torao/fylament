extern crate gif;

use std::fs::File;

use gif::SetParameter;

use crate::codec::{Codec, Decoder, Encoder};
use crate::error::FylmError;
use crate::image::{IndexedImage, RasterImage};

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
  fn read(&self, file: &mut File) -> Result<&dyn RasterImage, FylmError> {
    let mut decoder = gif::Decoder::new(file);
    decoder.set(gif::ColorOutput::RGBA);
    let mut decoder = decoder.read_info()?;

    /*
    while let Some(frame) = decoder.read_next_frame().unwrap() {
      let palette = decoder.palette();
      let width = frame.width as usize;
      let height = frame.height as usize;
      IndexedImage {
        width: width,
        height: height,
        palette: palette,
      }
    }
    */

    unimplemented!()
  }
}

#[derive(Debug)]
struct GIFEncoder;

impl Encoder for GIFEncoder {
  fn write(&self, image: &dyn RasterImage, file: &mut File) -> Result<u64, FylmError> {
    unimplemented!()
  }
}
