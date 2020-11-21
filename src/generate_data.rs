
use std::time::{SystemTime, Duration};
pub mod payload{
use std::process::{Command, Stdio};
use std::string::String;
extern crate serde;
extern crate serde_json;
extern crate parsers;
use parsers::parse::{parse_system,parse_network,parse_mobile,parse_wireless};
use serde_json::Value as JsonValue;
use serde_json::json;
use std::time::{SystemTime, UNIX_EPOCH};
    pub fn get_mobile() ->JsonValue
    {   
        let stdout = getStdout(vec!["mobiled.radio","signal_quality"]);
        let result:Result<JsonValue, serde_json::Error> = serde_json::from_str(&stdout);
        if result.is_ok()
        {
            return parse_mobile(&result.unwrap())
        }
        else
        {
            return json!({});
        }
    }
    pub fn get_data() -> Vec<JsonValue>
    {
        let stdout_mobile:Result<JsonValue, serde_json::Error> = serde_json::from_str(&getStdout(vec!["mobiled.radio","signal_quality"]));
        let stdout_system:Result<JsonValue, serde_json::Error> = serde_json::from_str(&getStdout(vec!["system","info"]));
        let stdout_network:Result<JsonValue, serde_json::Error> = serde_json::from_str(&getStdout(vec!["network.device","status"]));
        let stdout_wireless:Result<JsonValue, serde_json::Error> = serde_json::from_str(&getStdout(vec!["wireless.radio.stats","get"]));
        let mut my_vec: Vec<JsonValue> = Vec::new();
        if stdout_mobile.is_ok()
        {
            let station_mobile_json: JsonValue = parse_mobile(&stdout_mobile.unwrap());
            my_vec.push(station_mobile_json);
        }
        if stdout_system.is_ok()
        {
            let system_json: JsonValue = parse_system(&stdout_system.unwrap());
            my_vec.push(system_json);
        }
        if stdout_network.is_ok()
        {
            let network_json: Vec<JsonValue> = parse_network(&stdout_network.unwrap());
            for item in &network_json{
                my_vec.push(item.to_owned());
            }
        }
        if stdout_wireless.is_ok()
        {
            let wireless_json: Vec<JsonValue> = parse_wireless(&stdout_wireless.unwrap());
            for item in &wireless_json{
                my_vec.push(item.to_owned());
            }
        }
        return my_vec;

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

    pub fn generate_data(message_number:i32,s_time:&SystemTime,version:&str) -> String
    {
        let data: Vec<JsonValue> = get_data();
        return json!({
            "collector": "ModemStats",
            "data": data,
            "destination": "MQTT",
            "language": "Rust",
            "messages": data.len(),
            "platform": "armv7",
            "timestamp": SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            "uptime": s_time.elapsed().unwrap().as_secs(),
            "version": version
        }).to_string();
    }
}
#[test]
fn test_single_get()
{
    println!("{}",payload::get_mobile().to_string());
    assert!(payload::get_mobile().pointer("/fields").unwrap().is_object());
}

#[test]
fn test_get_data()
{
    println!("{:?}",payload::get_data());
}


#[test]
fn test_headers()
{
    let stime:SystemTime = SystemTime::now();
    //stime.elapsed().unwrap().as_secs()
    let data = payload::get_data();
    for i in vec![1,2]
    {
        println!("{}",payload::generate_data(i, &stime, "1.2.3"));
        std::thread::sleep(Duration::from_secs(5));
    }

}