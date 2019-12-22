extern crate clap;
extern crate gif;

use std::env;
use std::fs::File;
use std::io;

use clap::{App, Arg, SubCommand};
use gif::SetParameter;

fn main() {
  let matches = App::new("fylm")
      .version("0.1.0")
      .author("TAKAMI Torao <koiroha@gmail.com>")
      .about("film maker")
      .arg(Arg::with_name("inspect")
          .short("i")
          .long("inspect")
          .value_name("FILE")
          .help("Inspect specified GIF file.")
          .takes_value(true))
      .get_matches();

  match matches.value_of("inspect") {
    Some(file) => {
      inspect(file.to_string()).unwrap();
      return;
    }
    None => ()
  }

  let args: Vec<String> = env::args().collect();
  println!("args: {:?}", args);
}

fn inspect(file_name: String) -> Result<(), io::Error> {
  let file = File::open(file_name)?;

  let mut decoder = gif::Decoder::new(file);
  decoder.set(gif::ColorOutput::RGBA);

  let mut decoder = decoder.read_info().unwrap();
  println!("Logical Screen Size: {}x{} pixel", decoder.width(), decoder.height());
  match decoder.global_palette() {
    Some(palette) => {
      print!("Global Color Table: [)");
      for i in 0..palette.len()/3-1 {
        print!("#{:02x}{:02x}{:02x}", palette[i], palette[i+1], palette[i+2]);
      }
      println!("] ({} bytes)", palette.len());
    }
    None => ()
  }

  while let Some(frame) = decoder.read_next_frame().unwrap() {
    println!("  Base Point     : ({},{})", frame.left, frame.top);
    println!("  Size           : {}×{} pixel", frame.width, frame.height);
    println!("  Delay          : {}", frame.delay);
    println!("  Disposal Method: {:?}", frame.dispose);
    println!("  Interlaced     : {}", frame.interlaced);
    println!("  Need User Input: {}", frame.needs_user_input);
  }
  Ok(())
}