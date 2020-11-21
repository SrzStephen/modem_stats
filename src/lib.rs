#![crate_type = "lib"]
pub mod parse{
extern crate serde;
extern crate serde_json;
use serde_json::Value as JsonValue;
use serde_json::json;


    pub fn parse_wireless_interface(jval:&JsonValue,radio_interface:&str) -> JsonValue
    {
        return json!({
            "fields": json!({
                "tx_bytes": jval.pointer(&format!("/{}/tx_bytes",radio_interface)).unwrap().as_i64().unwrap(),
                "rx_bytes": jval.pointer(&format!("/{}/rx_bytes",radio_interface)).unwrap().as_i64().unwrap()
            }),
            "tags": json!({
                "interface": format!("/{}/rx_bytes",radio_interface),
                "interface_type": "logical"
            })
        })
    }
    
    pub fn parse_wireless(jval:&JsonValue) -> Vec<JsonValue>
    {
        let data_vec:Vec<JsonValue> = vec!["radio_2G","radio_5G"]
        .into_iter()
        .map(|x| parse_wireless_interface(&jval, &x))
        .rev()
        .collect();
        return data_vec;
    }

    pub fn parse_mobile(jval:&JsonValue) -> JsonValue
    {
        return json!(
            {
                "fields": json!({
                    "rsrp": jval.pointer("/rsrp").unwrap().as_i64().unwrap(),
                    "rssi": jval.pointer("/rssi").unwrap().as_i64().unwrap(),
                    "rsrq": jval.pointer("/rsrq").unwrap().as_i64().unwrap(),
                    "sinr": jval.pointer("/sinr").unwrap().as_i64().unwrap(),
                }),
                "tags": json!({
                    "interface": "LTE",
                    "interface_type": "logical"
                })
            }
    )
    }
    pub fn parse_system(jval:&JsonValue) -> JsonValue
    {
    return json!(
        {
            "fields": json!({
                "load_1": jval.pointer("/load/0").unwrap().as_i64().unwrap(),
                "load_5": jval.pointer("/load/1").unwrap().as_i64().unwrap(),
                "load_15": jval.pointer("/load/2").unwrap().as_i64().unwrap(),
                "uptime": jval.pointer("/uptime").unwrap().as_i64().unwrap(),
                "memory_total": jval.pointer("/memory/total").unwrap().as_i64().unwrap(),
                "memory_free": jval.pointer("/memory/free").unwrap().as_i64().unwrap(),
                "memory_shared": jval.pointer("/memory/shared").unwrap().as_i64().unwrap(),
                "memory_buffered": jval.pointer("/memory/buffered").unwrap().as_i64().unwrap(),
            }),
            "tags": json!({
            })
        })
}
pub fn parse_network_interface(jval:&JsonValue,interface:&str) -> JsonValue
{
    return json!(
         {
             "fields": json!({
                 "interface_present": jval.pointer(&format!("/{}/present",interface)).unwrap().as_bool().unwrap(),
                 "interface_up": jval.pointer(&format!("/{}/up",interface)).unwrap().as_bool().unwrap(),
                 "rx_bytes": jval.pointer(&format!("/{}/statistics/rx_bytes",interface)).unwrap().as_i64().unwrap(),
                 "tx_bytes": jval.pointer(&format!("/{}/statistics/tx_bytes",interface)).unwrap().as_i64().unwrap(),
             }),
             "tags": json!({
                 "interface": interface,
                 "interface_type": "physical"
             })
         })
 }
pub fn parse_network(jval:&JsonValue) -> Vec<JsonValue>
{
    let data_vec:Vec<JsonValue> = vec!["eth1","eth2","eth3","eth4","lo","wl0","wl0_1","wl0_2","wl0_3","wl1","wl1_1","wl1_2","wl1_3"]
    .into_iter()
    .map(|interface| parse_network_interface(&jval,&interface))
    .rev()
    .collect();
    return data_vec

}

}
