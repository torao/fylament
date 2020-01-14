type ARGB = u32;

/// Simple RGB color model with alpha channel. Each field indicates the strength of the
/// corresponding color channel in range 0-255. An alpha channel of 255 means complete opacity.
#[derive(Debug)]
pub struct Color {
  pub alpha: u8,
  pub red: u8,
  pub green: u8,
  pub blue: u8,
}

impl Color {
  /// Create a new ARGB color.
  pub fn new(r: u8, g: u8, b: u8, a: u8) -> Color {
    Color { alpha: a, red: r, green: g, blue: b }
  }

  /// Create a new RGB color as α=255.
  pub fn new_opaque(r: u8, g: u8, b: u8) -> Color {
    Color::new(r, g, b, 255)
  }

  /// Create a new RGB color from the specified integer that represents ARGB.
  pub fn new_from_argb(argb: ARGB) -> Color {
    Color::new((argb >> 16) as u8, (argb >> 8) as u8, (argb >> 0) as u8, (argb >> 24) as u8)
  }

  /// Create a new RGB color from the specified integer that represents RGB.
  pub fn new_from_rgb(rgb: ARGB) -> Color {
    Color::new_from_argb((0xFF << 24) | rgb)
  }

  /// Transform to 32-bit ARGB integer.
  pub fn to_argb(&self) -> ARGB {
    ((self.alpha as u32 & 0xFF) << 24)
        | ((self.red as u32 & 0xFF) << 16)
        | ((self.green as u32 & 0xFF) << 8)
        | ((self.blue as u32 & 0xFF) << 0)
  }
}

impl ToString for Color {
  fn to_string(&self) -> String {
    if self.alpha == 0xFF {
      format!("#{:02X}{:02X}{:02X}", self.red, self.green, self.blue)
    } else {
      format!("#{:02X}{:02X}{:02X}{:02X}", self.alpha, self.red, self.green, self.blue)
    }
  }
}

/// 2-dimensional ARGB bitmap image depends on image format.
pub trait RasterImage {
  fn width(&self) -> usize;
  fn height(&self) -> usize;
  fn pixel(&self, x: usize, y: usize) -> ARGB;

  fn get_line(&self, y: usize, line: &mut Vec<ARGB>) {
    let width = self.width();
    for x in 0..width - 1 {
      line[x] = self.pixel(x, y);
    }
  }
}

pub struct IndexedImage {
  width: usize,
  height: usize,
  palette: Vec<ARGB>,
  pixels: Vec<u8>,
}

impl RasterImage for IndexedImage {
  fn width(&self) -> usize { self.width }

  fn height(&self) -> usize { self.height }

  fn pixel(&self, x: usize, y: usize) -> ARGB {
    let index: u8 = self.pixels[y * self.width + x];
    self.palette[index as usize]
  }
}



/*
pub trait Screen {
  fn size(&self) -> Dimension2D;
  fn raster_image(&self) -> *const RasterImage;
}

/// The Size represents a 2-dimensional rectangle size.
#[derive(Debug)]
pub struct Size {
  width: u32,
  height: u32,
}

/// 2-dimensional ARGB bitmap image.
#[derive(Debug)]
pub struct RasterImage {
  size: Size,
  raster: Vec<Vec<u32>>,
}

/*
pub trait Screen {
  fn size(&self) -> Dimension2D;
  fn raster_image(&self) -> *const RasterImage;
}

#[derive(Debug)]
pub struct Story {}

#[derive(Debug)]
pub struct Scene {}

#[derive(Debug)]
pub struct Shot {
  frames: *const Vec<Frame>,
}

#[derive(Debug)]
pub struct Frame {
  screen: *const dyn Screen,
}

impl Screen for Frame {
  fn size(&self) -> Dimension2D {
    self.raster_image().size()
  }

  fn raster_image(&self) -> *const RasterImage {
    unimplemented!()
  }
}
*/
