pub mod payload{
use std::process::{Command, Stdio};
use std::string::String;
extern crate json;

    pub fn make_payload() -> String{
        //let mystr: String  = getStdout(vec!["mobiled.radio","signal_quality"]);
        let mut data = json::JsonValue::new_object();
        data["mobile"] = json::parse(&getStdout(vec!["mobiled.radio","signal_quality"])).unwrap();
        data["system"] = json::parse(&getStdout(vec!["system","info"])).unwrap();
        data["wireless"] = json::parse(&getStdout(vec!["wireless.radio.stats","get"])).unwrap();
        data["network"] = json::parse(&getStdout(vec!["network.device","status"])).unwrap();
        return json::stringify_pretty(data,4);
    }
    
    fn getStdout(args:Vec<&str>) -> String {
        let cmd = Command::new("ubus")
        .arg("call")
        .args(&args)
        .stdout(Stdio::piped())
        .output()
        .unwrap();
        return String::from_utf8(cmd.stdout).unwrap();
    }
}
