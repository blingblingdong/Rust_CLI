use std::collections::HashMap;
use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::ops::ControlFlow;
use std::ptr::replace;

type MyResult<T> = Result<T, Box<dyn Error>>;


#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: Option<isize>,
    bytes: Option<isize>,
    chars: Option<isize>,
    quiet: bool,
}

pub fn run(config: Config) -> MyResult<()> {
    let num_files = config.files.len();
    for file_name in config.files {
        match open(&file_name) {
            Ok(file) => {
                if !config.quiet && num_files > 1 {
                    println!("==> {} <==", &file_name);
                }
                let lines: Vec<_> = file.lines().collect::<Result<_, _>>()?;
                // let take_line = test_number(config.lines, lines.len() as isize)?;
                match (config.chars, config.bytes, config.lines) {
                    (_, _, Some(config_line)) => {
                        let take_line = test_number(config_line, lines.len() as isize)?;
                        for line in lines.iter().take(take_line) {
                            match (config.chars, config.bytes) {
                                (Some(chars), None) => {
                                    let take_chars = test_number(chars, line.chars().count() as isize)?;
                                    let string_chars: String = line.chars().take(take_chars).collect();
                                    println!("{string_chars}");
                                },
                                (None, Some(bytes)) => {
                                    let take_bytes = test_number(bytes, line.bytes().count() as isize)?;
                                    let bytes_vec: Vec<u8> = line.bytes().take(take_bytes).collect();
                                    println!("{}", String::from_utf8_lossy(&bytes_vec));
                                },
                                (_, _) => {
                                    println!("{line}");
                                }
                            }
                        }
                    },
                    (Some(chars), None, None) => {
                        let mut s = String::new();
                        for line in lines{
                            s.push_str(&line);
                            s.push_str("\n");
                        };
                        let take_chars = test_number(chars, s.chars().count() as isize)?;
                        let print_chars: String = s.chars().take(take_chars).collect();
                        println!("{print_chars}");
                    },
                    (None, Some(bytes), None) => {
                        let mut s = String::new();
                        for line in lines{
                            s.push_str(&line);
                            s.push_str("\n");
                        };
                        let take_bytes = test_number(bytes, s.bytes().count() as isize)?;
                        let bytes_vec: Vec<u8> = s.bytes().take(take_bytes).collect();
                        println!("{}", String::from_utf8_lossy(&bytes_vec));
                    },
                    (_, _, _) => {
                        for line in lines.iter().take(10) {
                            println!("{line}");
                        }
                    }
                }
                println!();
            }
            Err(e) => eprintln!("{}:{}", &file_name, e),
        }
    }
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("headr")
        .version("0.1.0")
        .author("Bling-Bling-Dong <whoareyouman0.4@gmail.com>")
        .about("以Rust驅動的head")
        .arg(
            Arg::with_name("text")
                .value_name("Text")
                .help("輸入檔案，預設為-")
                .multiple(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("lines")
                .value_name("Lines")
                .short("n")
                .help("顯示行數，預設為10")
                .allow_hyphen_values(true),
        )
        .arg(Arg::with_name("chars")
                .value_name("Chars")
                .short("c")
                .help("顯示字元組")
                .allow_hyphen_values(true),
        )
        .arg(
            Arg::with_name("quiet")
                .short("q")
                .help("不顯示檔案頭")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("bytes")
                .value_name("Bytes")
                .short("b")
                .help("顯示位元組，不可與字元並用")
                .conflicts_with("chars")
                .allow_hyphen_values(true)
        )
        .get_matches();

    let bytes = matches
        .value_of("bytes")
        .map(|c|c.parse())
        .transpose()
        .map_err(|e| format!("illegal bytes count -- {}", e))?;

    let chars = matches
        .value_of("chars") //Option<Value>
        .map(|c| c.parse())
        .transpose()
        .map_err(|e| format!("illegal chars count -- {}", e))?;

    let lines = matches
        .value_of("lines")
        .map(|b| b.parse())
        .transpose()
        .map_err(|e| format!("illegal line count -- {}", e))?;

    Ok(Config {
        files: matches.values_of_lossy("text").unwrap(),
        lines,
        chars,
        bytes,
        quiet: matches.is_present("quiet"),
    })
}

// 對於dyn Bufead編譯器無法知道大小，所以無法儲存在對堆疊上
// 解法是配置堆疊記憶體，將回傳值放入Box中
fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn test_number(val: isize, lines: isize) -> MyResult<usize> {
    use std::cmp::Ordering::*;
    match (val.cmp(&0), val.cmp(&lines)) {
        (Less, _) if -val <= lines => Ok((lines + val) as usize),
        (Greater, Less) => Ok(val as usize),
        (Greater, Equal) => Ok(lines as usize),
        (_, _) => Ok(lines as usize),
    }
}

fn tt()  {
    let ugly_lines = vec!["Rust是一門安全的語言","米老鼠花栗鼠吉瑞鼠", "Java是一門安全的語言", "Rust用嚴格的編譯減少運行錯誤","C++哈哈哈", "Java用物件型態來建構大型應用","Rsut被用來改寫很多底層程式","Java常見於安著應用開發","pypypu"];

    let (mut rust_lines, mut java_lines) : (Vec<&str>, Vec<&str>)
        = ugly_lines.into_iter()
        .filter(|&line| line.contains("Java") || line.contains("Rust"))
        .partition(|&line| line.contains("rust"));

    println!("關於Rust：");
    rust_lines.iter()
        .enumerate()
        .map(|(i, line)| {format!("{}:{}", i, line.replace("Rust", ""))})
        .for_each(|line| println!("{line}"));



}





