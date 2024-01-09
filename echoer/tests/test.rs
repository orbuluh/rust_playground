use assert_cmd::Command; // need to add dev-dependencies in Cargo.toml
use std::fs;

#[test]
fn dies_no_args() {
  let mut cmd = Command::cargo_bin("echoer").unwrap();
  cmd.assert().failure().stderr(predicates::str::contains("USAGE"));
}


#[test]
fn runs() {
  let mut cmd = Command::cargo_bin("echoer").unwrap();
  cmd.arg("hello").assert().success();
}

#[test]
fn compared_to_file_1() {
  let outfile = "tests/expected/hello1.txt";
  // Using fs::read_to_string is a convenient way to read a file into
  // memory, but it’s also an easy way to crash your program—and pos‐
  // sibly your computer—if you happen to read a file that exceeds your
  // available memory. You should only use this function with small
  // files. 
  let expected_res = fs::read_to_string(outfile).unwrap();
  let mut cmd = Command::cargo_bin("echoer").unwrap();
  cmd.arg("Hello there").assert().success().stdout(expected_res);
}