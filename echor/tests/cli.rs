use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn dies_no_args() -> TestResult {
  let mut cmd = Command::cargo_bin("echor")?;
  cmd.assert()
      .failure()
      .stderr(predicate::str::contains("USAGE"));
  Ok(())
}

#[test]
fn runs() -> TestResult {
  let mut cmd = Command::cargo_bin("echor")?;
  cmd.arg("hello").assert().success();
  Ok(())
}

fn run(arg: &[&str], expected_file: &str) -> TestResult {
   let expected = fs::read_to_string(expected_file)?;
   Command::cargo_bin("echor")?
      .args(arg)
      .assert()
      .success()
      .stdout(expected);
      
   Ok(())
}

#[test]
fn hello() -> TestResult {
  let args = vec!(vec!["Hello there"], vec!["Hello", "there"], vec!["Hello", "there", "-n"], vec!["-n", "Hello", "there"]);

  let path = "tests/expected/";
  let files = ["hello1.txt", "hello2.txt", "hello1.n.1", "hello2.n.txt"];
  let file_vec: Vec<_> = 
    files.iter()
    .map(|s| path.to_owned() + *s )
    .zip(&args)
    .collect();
    
  for (file, arg) in file_vec {
    run(&arg, &file);
  }
  
  Ok(())
  
}