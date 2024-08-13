use rand::{distributions::Alphanumeric, Rng};
use predicates::prelude::*;
use assert_cmd::Command;
use std::fs;
type TestResult = Result<(), Box<dyn std::error::Error>>;

const RPG: &str = "catr";
const EMPTY: &str = "tests/inputs/empty.txt";
const FOX: &str = "tests/inputs/fox.txt";
const SPIDERS: &str = "tests/inputs/spiders.txt";
const BUSTLE: &str = "tests/inputs/the-bustle.txt";


fn gen_bad_file() -> String {
  loop {
    let file_name = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();
        
    // 當參數的file不存在，metadata會發生錯誤
    if fs::metadata(&file_name).is_err() {
      return file_name;
    }
  }
}

#[test]
fn skip_bad_file() -> TestResult {
  let bad = gen_bad_file();
  let expected = format!("{}: .* [(]os error 2[)]", bad);
  Command::cargo_bin(RPG)?
      .arg(bad)
      .assert()
      .success()
      .stderr(predicate::str::is_match(expected)?);
      
  Ok(())
}


