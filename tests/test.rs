use std::process::{Command, Stdio};
use assert_cmd::prelude::{CommandCargoExt, OutputAssertExt};
extern crate serde_json;
use serde_json::json;
use serde_json::Value as JsonValue;
#[test]
fn test_cli_help(){
let cmd = Command::new("ModemStats")
.args(&["--help"])
.stdout(Stdio::piped()).output().unwrap();
println!("TEST");
println!("{}",cmd.status);
println!("STDOUT IS {}",String::from_utf8(cmd.stdout).unwrap());
assert_eq!(cmd.stderr.len(),0);
}
#[test]
fn test_cli_works()
{
let cmd = Command::new("ModemStats")
.args(&["stdout"])
.stdout(Stdio::piped()).output().unwrap();
println!("stderror is {}", String::from_utf8(cmd.stdout).unwrap());
assert_eq!(cmd.stderr.len(),0);
}
#[test]
fn test_cli_is_json()
{
    let cmd = Command::new("ModemStats")
    .args(&["stdout"])
    .stdout(Stdio::piped()).output().unwrap();
    assert_eq!(cmd.stderr.len(),0);
    let res:Result<JsonValue, serde_json::Error> = serde_json::from_str(&format!("{}",String::from_utf8(cmd.stdout).unwrap().to_string()));
    if res.is_err(){
        panic!("It's not json");
    }
}