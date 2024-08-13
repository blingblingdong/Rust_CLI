use std::error::Error;
use clap::{Arg, App};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
  let files = config.files;
  for file_name in &files {
    match open(&file_name) {
      Err(e) => eprintln!("{file_name}: {e}"),
      Ok(file) => {
        let mut count = 1;
        for (i, line_result) in file.lines().enumerate() {
          let line = line_result?;
          if config.number_lines {
            println!("{:>6}\t{}", i+1, line);
          } else if config.number_noblank_lines {
            if line.is_empty() {
              println!(" ");
            } else {
               println!("{:>6}\t{}", count, line);
               count += 1
            }
          } else {
            println!("{}", line);
          }
        }
      }
    }
  }
  Ok(())
}

#[derive(Debug)]
pub struct Config {
  files: Vec<String>,
  number_lines: bool,
  number_noblank_lines: bool,
}

pub fn get_args() -> MyResult<Config> {
  let matches = App::new("catr")
      .version("0.1.0")
      .author("Bling-Bling-Dong <whoareyouman0.4@gmail.com>")
      .about("以Rust驅動的cat")
      .arg (
          Arg::with_name("text")
              .value_name("Text")
              .help("輸入檔案，預設為-")
              .multiple(true)
              .default_value("-"),
      )
      .arg(
          Arg::with_name("number")
              .short("n")
              .help("Number lines")
              .takes_value(false)
              .conflicts_with("number_nonblank"),
         )
      .arg(
          Arg::with_name("number_nonblank")
              .short("b")
              .help("Number-nonblank")
              .takes_value(false),
      )
      .get_matches();
  
      
  Ok(Config {
    files: matches.values_of_lossy("text").unwrap() ,
    number_lines: matches.is_present("number"),
    number_noblank_lines: matches.is_present("number_nonblank"),
  })
}

// 對於dyn Bufead編譯器無法知道大小，所以無法儲存在對堆疊上
// 解法是配置堆疊記憶體，將回傳值放入Box中
fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
  match filename {
    "-" => Ok(Box::new(BufReader::new(io::stdin()))),
    _ => Ok(Box::new(BufReader::new(File::open(filename)?)))
  }
}




