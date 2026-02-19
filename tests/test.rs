use std::fs;
use predicates::prelude::*;
use assert_cmd::Command;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn dies_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin("recho")?;
    cmd.assert().failure().stderr(predicate::str::contains("USAGE"));
    Ok(())
}

#[test]
fn program_runs() -> TestResult {
    let mut cmd = Command::cargo_bin("recho")?;
    cmd.arg("hello").assert().success();
    Ok(())
}

fn runs(args: &[&str], file: &str) -> TestResult {
    let expected_file = fs::read_to_string(file)?;
    Command::cargo_bin("recho")?.args(args).assert().success().stdout(expected_file);
    Ok(())
}

#[test]
fn hello1() -> TestResult{
    runs(&["Hello there"], "tests/expected/hello1.txt")
} 
#[test]
fn hello2() -> TestResult{
    runs(&["Hello", "there"], "tests/expected/hello2.txt")
}
#[test]
fn hello1n() -> TestResult{
    runs(&["Hello  there", "-n"], "tests/expected/hello1.n.txt")
}
#[test]
fn hello2n() -> TestResult{
    runs(&["-n", "Hello", "there"], "tests/expected/hello2.n.txt")
}