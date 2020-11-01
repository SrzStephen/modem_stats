#[macro_use]
extern crate clap;
use clap::{App};
use aws_iot_device_sdk_rust::{client,QoS};
use std::{thread, time};
use log::{info, trace, warn};
use std::process;
mod generate_data;
pub use crate::generate_data::payload;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let m = App::from(yaml).get_matches();
    if let Some(matches) = m.subcommand_matches("stdout")
    {
        print!("\n REACHED STDOUT");
        let period: u64 = matches.value_of("period").unwrap().parse::<u64>().unwrap();
        let bool_continue: bool = matches.occurrences_of("continue")> 0; //treat it as a flag
        stdout(bool_continue,time::Duration::from_secs(period));

    }
    if let Some(matches) = m.subcommand_matches("mqtt")
    {
        println!("Trying MQTT");
        let period: u64 = matches.value_of("period").unwrap().parse::<u64>().unwrap();
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
    print!("starting to wait\n");
        thread::sleep(time::Duration::from_secs(2));
    print!("Waited");
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
    while true {
        iot_client.publish(topic.to_string(),QoS::AtMostOnce, &payload::make_payload());
        println!("Publishing to topic {}",topic.to_string());
        thread::sleep(wait_duration);
    }
}

fn stdout(cont: bool, period: time::Duration){
    // Not using generators becuase they're super experimental atm
    while true {
        print!("Starting loop");
        print!("{}",payload::make_payload());
        //print!("{}",payload::make_common_payload()); //Just test any command
        if cont != true{
            process::exit(0);
        }
        thread::sleep(period);
    }  
    }
    



