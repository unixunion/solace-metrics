extern crate influx_db_client;

use solace_semp_client_monitor::apis::client::APIClient;
use hyper_tls::HttpsConnector;
use hyper::client::HttpConnector;
use tokio_core::reactor::Core;
use influx_db_client::{Point, Client};
use std::collections::HashMap;
use serde_json::Value;
use influx_db_client::Client as InfluxClient;
use influx_db_client::Value as InfluxValue;
use solace_semp_client_monitor::models::{MsgVpnResponse, MsgVpnClientConnectionsResponse, MsgVpnClientResponse, MsgVpnClientsResponse, MsgVpnQueueResponse};
use std::collections::hash_map::RandomState;
use serde::Serialize;

pub trait Metric<T> {

//    fn get(item_name: &str, subitem_name: &str, selector: &str, apiclient: &APIClient<HttpsConnector<HttpConnector>>, core: &mut Core) -> Result<T, &'static str>;
//    fn create_metric(point: &str, item: &T, tags: HashMap<String, String>, influxdb_client: &mut InfluxClient) -> Point;
//    fn extract_data(item: &T) -> Point;
    fn get(&self, apiclient: &APIClient<HttpsConnector<HttpConnector>>, core: &mut Core) -> Result<T, &'static str>;
    fn create_metric(point: &str, item: &T, tags: HashMap<String, String>, influxdb_client: &mut InfluxClient) -> Vec<Point>;
    fn extract_data(item: &T) -> Vec<Point> where T: Serialize;

    /*
    make fields directly off the root of JSON, and decend one level if object in value.
    */
    fn make_fields(&self, data: String, measurement_name: &str) -> Result<Point, &'static str>;

//    {
//
//        let mut point = Point::new(measurement_name);
//
//        info!("string: {:?}", data);
//
//        let t: Vec<Value> = serde_json::from_str(&data).unwrap()?;
//
////        debug!("type {}", serde_json::from_str(&data).unwrap());
//        let t: HashMap<String, Value> = serde_json::from_str(&data).unwrap()?;
//
//        for (k,v) in t.into_iter() {
//            debug!("key: {:?} value: {:?}", k, v);
//
//            match v {
//                // descend into objets
//                Value::Object(obj) => {
//                    for (ok, ov) in obj {
//                        let key = format!("{}_{}", k, ok);
//                        info!("{:?} {:?}", key, ov);
//                        point.add_field(key, InfluxValue::Integer(ov.as_i64().unwrap()));
//                    }
//                },
//                // add keys with number values
//                Value::Number(num) => {
//                    info!("{:?} {:?}", k, num);
//                    point.add_field(k, InfluxValue::Integer(num.as_i64().unwrap()));
//                },
//                // skip anything else
//                _ => {
//                    debug!("skipping: {:?}, {:?}", k, v);
//                }
//            }
//
//        }
//        Ok(point)
//
//    }

}


impl Metric<MsgVpnResponse> for MsgVpnResponse {

    fn get(&self, apiclient: &APIClient<HttpsConnector<HttpConnector>>, core: &mut Core) -> Result<MsgVpnResponse, &'static str> {
        unimplemented!()
    }

    fn create_metric(point: &str, item: &MsgVpnResponse, tags: HashMap<String, String, RandomState>, influxdb_client: &mut Client) -> Vec<Point> {
        unimplemented!()
    }


    fn extract_data(item: &MsgVpnResponse) -> Vec<Point> where MsgVpnResponse: Serialize {
        match serde_json::to_string(item.data().unwrap()) {
            Ok(s) => {
                match item.make_fields(s, "message-vpn") {
                    Ok(p) => {
                        vec![p]
                    }
                    _ => {
                        unimplemented!()
                    }
                }
            }
            _ => {
                unimplemented!()
            }
        }
    }

    fn make_fields(&self, data: String, measurement_name: &str) -> Result<Point, &'static str> {

            let mut point = Point::new(measurement_name);

            info!("string: {:?}", data);

//            let t: Vec<Value> = serde_json::from_str(&data).unwrap()?;

//        debug!("type {}", serde_json::from_str(&data).unwrap());
            let t: HashMap<String, Value> = serde_json::from_str(&data).unwrap();

            for (k,v) in t.into_iter() {
                debug!("key: {:?} value: {:?}", k, v);

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
                        debug!("skipping: {:?}, {:?}", k, v);
                    }
                }

            }
            Ok(point)


    }
}


// queue
impl Metric<MsgVpnQueueResponse> for MsgVpnQueueResponse {

    fn get(&self, apiclient: &APIClient<HttpsConnector<HttpConnector>>, core: &mut Core) -> Result<MsgVpnQueueResponse, &'static str> {
        unimplemented!()
    }

    fn create_metric(point: &str, item: &MsgVpnQueueResponse, tags: HashMap<String, String, RandomState>, influxdb_client: &mut Client) -> Vec<Point> {
        unimplemented!()
    }


    fn extract_data(item: &MsgVpnQueueResponse) -> Vec<Point> where MsgVpnQueueResponse: Serialize {
        match serde_json::to_string(item.data().unwrap()) {
            Ok(s) => {
                match item.make_fields(s, "message-vpn-queue") {
                    Ok(p) => {
                        vec![p]
                    }
                    _ => {
                        unimplemented!()
                    }
                }
            }
            _ => {
                unimplemented!()
            }
        }
    }

    fn make_fields(&self, data: String, measurement_name: &str) -> Result<Point, &'static str> {

        let mut point = Point::new(measurement_name);

        info!("string: {:?}", data);

//            let t: Vec<Value> = serde_json::from_str(&data).unwrap()?;

//        debug!("type {}", serde_json::from_str(&data).unwrap());
        let t: HashMap<String, Value> = serde_json::from_str(&data).unwrap();

        for (k,v) in t.into_iter() {
            debug!("key: {:?} value: {:?}", k, v);

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
                    debug!("skipping: {:?}, {:?}", k, v);
                }
            }

        }
        Ok(point)


    }
}


impl Metric<MsgVpnClientsResponse> for MsgVpnClientsResponse {

    fn get(&self, apiclient: &APIClient<HttpsConnector<HttpConnector>>, core: &mut Core) -> Result<MsgVpnClientsResponse, &'static str> {
        unimplemented!()
    }

    fn create_metric(point: &str, item: &MsgVpnClientsResponse, tags: HashMap<String, String, RandomState>, influxdb_client: &mut Client) -> Vec<Point> {
        unimplemented!()
    }

    fn extract_data(item: &MsgVpnClientsResponse) -> Vec<Point> where MsgVpnClientsResponse: Serialize {

        let mut points = Vec::new();

        match serde_json::to_string(item.data().unwrap()) {
            Ok(s) => {
                match item.make_fields(s, "client") {
                    Ok(p) => {
                        points.push(p)
                    }
                    _ => {
                        unimplemented!()
                    }
                }
            }
            _ => {
                unimplemented!()
            }
        }

        info!("points: {:?}", points);

        points
    }

    fn make_fields(&self, data: String, measurement_name: &str) -> Result<Point, &'static str> {
        let mut point = Point::new(measurement_name);

        info!("string: {:?}", data);

        let t: Vec<Value> = serde_json::from_str(&data).unwrap();

        for i in t.into_iter() {

            info!(" value: {:?}", i);

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

//            match v {
//                // descend into objets
//                Value::Object(obj) => {
//                    for (ok, ov) in obj {
//                        let key = format!("{}_{}", k, ok);
//                        info!("{:?} {:?}", key, ov);
//                        point.add_field(key, InfluxValue::Integer(ov.as_i64().unwrap()));
//                    }
//                },
//                // add keys with number values
//                Value::Number(num) => {
//                    info!("{:?} {:?}", k, num);
//                    point.add_field(k, InfluxValue::Integer(num.as_i64().unwrap()));
//                },
//                // skip anything else
//                _ => {
//                    debug!("skipping: {:?}, {:?}", k, v);
//                }
//            }

        }
        Ok(point)
    }
}