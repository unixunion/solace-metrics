extern crate influx_db_client;

use solace_semp_client_monitor::apis::configuration::BasicAuth;
use colored::*;
use log::{info};
use std::fs::File;
use std::io::prelude::*;
use futures::future::Ok;
use std::path::Path;
use std::error::Error;
use serde::{Serialize, Deserialize};
use std::any::Any;
use serde_json::Value;
use influx_db_client::{Point, Points};
use influx_db_client::Value as InfluxValue;


// generate a credential for basicauth
pub fn gencred(username: String, password: String) -> BasicAuth {
    let password: Option<String> = Some(password);
    BasicAuth::from((username, password ))
}

// build a where selector
pub fn getwhere(key: &str, name: &str, select: &str) -> (Vec<String>,Vec<String>) {
    let mut wherevec: Vec<String> = Vec::new();
    let whereitem = format!("{}=={}", key, name);
    wherevec.push(String::from(whereitem));

    let selectvec = getselect(select);

    debug!("generated wherevec: {:?} and selectvec: {:?}", &wherevec, &selectvec);

    (wherevec, selectvec)
}

pub fn getselect(select: &str) -> Vec<String> {
    // SEMP selector
    let mut selectvec: Vec<String> = Vec::new();
    selectvec.push(String::from(select));
    selectvec
}


pub fn getPoint(i: Value, measurement_name: &str) -> Point {

    let mut point = Point::new(measurement_name);

    for (k,v) in i.as_object().unwrap() {

        info!("key: {:?} value: {:?}", k, v);

        match v {
            // descend into objets
            Value::Object(obj) => {
                for (ok, ov) in obj {
                    let key = format!("{}_{}", k, ok);
                    info!("{:?} {:?}", key, ov);
                    point.add_field(key, InfluxValue::Integer(ov.as_i64().unwrap()));
                }
            },
            // add keys with number values
            Value::Number(num) => {
                info!("{:?} {:?}", k, num);
                point.add_field(k, InfluxValue::Integer(num.as_i64().unwrap()));
            },
            // skip anything else
            _ => {
                warn!("skipping: {:?}, {:?}", k, v);
            }
        }

    }

    point

}


