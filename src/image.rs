#[derive(Debug)]
pub struct Size {
  width: u32,
  height: u32,
}

pub trait Image {
  fn size(&self) -> &Size;
}

pub trait Screen {
  fn size(&self) -> &Size;
}

#[derive(Debug)]
pub struct RasterImage {
  size: Size,
}

#[delive(Debug)]
pub struct Story {}

#[delive(Debug)]
pub struct Scene {}

#[delive(Debug)]
pub struct Shot {
  frames: Vec<Frame>
}

#[delive(Debug)]
pub struct Frame {
  size: Size
}

impl Screen for Frame {
  fn size(&self) -> &Size { &self.size }
}