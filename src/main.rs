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

fn rgb(r: u8, g: u8, b: u8) -> String {
  format!("#{:02X}{:02X}{:02X}", r, g, b)
}

fn argb(a: u8, r: u8, g: u8, b: u8) -> String {
  format!("#{:02X}{:02X}{:02X}{:02X}", a, r, g, b)
}

fn inspect(file_name: String) -> Result<(), io::Error> {
  let file = File::open(file_name)?;

  let mut decoder = gif::Decoder::new(file);
  decoder.set(gif::ColorOutput::RGBA);

  let mut decoder = decoder.read_info().unwrap();
  println!("Logical Screen Size      : {}x{} pixel", decoder.width(), decoder.height());
  print!("Global Color Table       : ");
  match decoder.global_palette() {
    Some(palette) => {
      print!("[");
      for i in 0..(palette.len() / 3) {
        if i != 0 {
          print!(",");
        }
        print!("{}", rgb(palette[i * 3], palette[i * 3 + 1], palette[i * 3 + 2]));
      }
      println!("] ({} bytes, {} colors)", palette.len(), palette.len() / 3);
    }
    None => println!("-")
  }
  println!("Background Color Index   : {}",
           decoder.bg_color().map_or("-".to_string(), |x| x.to_string()));

  let mut i = 0;
  while let Some(frame) = decoder.read_next_frame().unwrap() {
    println!("  Image Position         : ({},{})", frame.left, frame.top);
    println!("  Image Size             : {}×{} pixel", frame.width, frame.height);
    println!("  Interlaced             : {}", frame.interlaced);
    println!("  Delay Time             : {}", frame.delay);
    println!("  Disposal Method        : {} ({:?})", frame.dispose as u8, frame.dispose);
    println!("  Transparent Color Index: {}",
             frame.transparent.map_or("-".to_string(), |x| x.to_string()));
    println!("  Need User Input        : {}", frame.needs_user_input);
    print!("  Image Data             : ");
    for i in 0..(frame.buffer.len() / 4) {
      if i != 0 {
        print!(",");
      }
      print!("{}", argb(frame.buffer[(i * 4) + 3], frame.buffer[(i * 4)], frame.buffer[(i * 4) + 1], frame.buffer[(i * 4) + 2]));
    }
    println!(" ({} bytes, {} pixels)", frame.buffer.len(), frame.buffer.len() / 4);
  }
  Ok(())
}