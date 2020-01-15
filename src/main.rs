extern crate clap;
extern crate gif;
extern crate fylm;

use std::env;
use std::fs::{File, read_to_string};
use std::io;

use clap::{App, Arg, SubCommand};
use gif::SetParameter;
use fylm::parser::{parser, ParseError};

fn main() {
  let matches = App::new("fylm")
    .version("0.1.0")
    .author("TAKAMI Torao <koiroha@gmail.com>")
    .about("fylm maker")
    .subcommand(SubCommand::with_name("inspect")
      .about("Inspect the specified GIF file.")
      .arg(Arg::with_name("gif_file")
        .value_name("FILE")
        .help("A GIF file in which to display properties.")
        .takes_value(true)
      )
    )
    .subcommand(SubCommand::with_name("build")
      .about("Build animation from the specified fylm file.")
      .arg(Arg::with_name("fylm_file")
        .value_name("FILE")
        .help("")
        .takes_value(true)
      )
    )
    .get_matches();

  match matches.subcommand() {
    ("inspect", Some(args)) =>
      match args.value_of("gif_file") {
        Some(file) => {
          inspect(file.to_string()).unwrap();
          return;
        }
        None => ()
      },
    ("build", Some(args)) => {}
    (unknown, _) => ()
  }

  let args: Vec<String> = env::args().collect();
  println!("args: {:?}", args);
}

fn build(file_name: String) -> Result<(), ParseError> {
  let content = read_to_string(file_name)?;
  parser(&content);
  Ok(())
}

fn inspect(file_name: String) -> Result<(), io::Error> {
  let file = File::open(file_name)?;

  let mut decoder = gif::Decoder::new(file);
  decoder.set(gif::ColorOutput::RGBA);

  fn rgb(r: u8, g: u8, b: u8) -> String {
    format!("#{:02X}{:02X}{:02X}", r, g, b)
  }

  fn argb(a: u8, r: u8, g: u8, b: u8) -> String {
    format!("#{:02X}{:02X}{:02X}{:02X}", a, r, g, b)
  }

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
    i += 1;
    println!("[#{}]", i);
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