#[macro_use]
extern crate clap;
use clap::{App};
use aws_iot_device_sdk_rust::{client,QoS};
use std::{thread, time};
use log::{info, trace, warn};
use std::process;
mod generate_data;
pub use crate::generate_data::payload;
use std::time::SystemTime;
static VERSION:&str = "1.1.0";

fn main() {
    let yaml = load_yaml!("cli.yml");
    let m = App::from(yaml).get_matches();
    if let Some(matches) = m.subcommand_matches("stdout")
    {
        let period: u64 = matches.value_of("period").unwrap().parse::<u64>().unwrap();
        let bool_continue: bool = matches.occurrences_of("continue")> 0; //treat it as a flag
        stdout(bool_continue,time::Duration::from_secs(period));

    }
    if let Some(matches) = m.subcommand_matches("mqtt")
    {        let period: u64 = matches.value_of("period").unwrap().parse::<u64>().unwrap();
        let clientid: &str = matches.value_of("client_id").unwrap();
        let rootCA: &str = matches.value_of("path_to_cert").unwrap();
        let devicecert: &str = matches.value_of("device_cert").unwrap();
        let devicekey: &str = matches.value_of("device_private_key").unwrap();
        let endpoint: &str = matches.value_of("endpoint_url").unwrap();
        let topic: &str = matches.value_of("topic").unwrap();
        println!("client {} \n root_ca {} \n device_cert {} \n device_key {} \n endpoint {} \n",
        clientid,rootCA,devicecert,devicekey,endpoint);
        mqtt(clientid,rootCA,devicecert,devicekey,endpoint,topic,
            time::Duration::from_secs(period))
    }
        // wait for services to come up properly on boot.
        thread::sleep(time::Duration::from_secs(300));
}

fn mqtt(clientid:&str ,rootCA: &str, devicecert:&str,devicekey:&str,endpoint:&str,topic:&str,wait_duration:time::Duration){
    println!("Starting client");
    let mut iot_client = client::AWSIoTClient::new(
        clientid,
        rootCA,
        devicecert,
        devicekey,
        endpoint
    ).unwrap();
    println!("Starting loop");
    let mut message_number:i32 = 0;
    let time_now = SystemTime::now();
    while true {
        iot_client.publish(topic.to_string(),QoS::AtMostOnce, &payload::generate_data(message_number, &time_now, VERSION));
        println!("Publishing to topic {}",topic.to_string());
        thread::sleep(wait_duration);
    }
}

fn stdout(cont: bool, period: time::Duration){
    // Not using generators becuase they're super experimental atm
    let mut message_number:i32 = 0;
    let time_now = SystemTime::now();
    while true {
        print!("{}",payload::generate_data(message_number, &time_now, VERSION));
                if cont != true{
            process::exit(0);
        }
        thread::sleep(period);
        message_number +=1;
    }  
    }
    



