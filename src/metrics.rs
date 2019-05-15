
extern crate influx_db_client;

use influx_db_client::{Point, Points, Precision};
use influx_db_client::Client as InfluxClient;
use influx_db_client::Value as InfluxValue;
use solace_semp_client_monitor::models::{MsgVpnResponse, MsgVpnsResponse, MsgVpnJndiQueueResponse};
use std::collections::HashMap;
use solace_semp_client_monitor::apis::client::APIClient;
use hyper_tls::HttpsConnector;
use hyper::client::HttpConnector;
use crate::helpers::getselect;
use tokio_core::reactor::Core;
use futures::future::Future;
use std::time::{SystemTime, UNIX_EPOCH};
use itertools::{Itertools, iterate};
use serde_json::Value;
use serde::Serialize;

mod test {

}


pub trait Metric<T> {

    fn get(item_name: &str, subitem_name: &str, selector: &str, apiclient: &APIClient<HttpsConnector<HttpConnector>>, core: &mut Core) -> Result<T, &'static str>;
    fn create_metric(point: &str, item: &T, tags: HashMap<String, String>, influxdb_client: &mut InfluxClient) -> Point;
    fn extract_data(item: &T) -> Point;

    /*
    make fields directly off the root of JSON, and decend one level if object in value.
    */
    fn make_fields(&self, data: String, measurement_name: &str) -> Result<Point, &'static str> {

        let mut point = Point::new(measurement_name);

        debug!("string: {:?}", data);
        let t: HashMap<String, Value> = serde_json::from_str(&data).unwrap();
        for (k,v) in t.into_iter() {
            debug!("key: {:?} value: {:?}", k, v);

            match v {
                Value::Object(obj) => {
                    for (ok, ov) in obj {
                        let key = format!("{}_{}", k, ok);
                        info!("{:?} {:?}", key, ov);
                        point.add_field(key, InfluxValue::Integer(ov.as_i64().unwrap()));
                    }
                },
                Value::Number(num) => {
                    info!("{:?} {:?}", k, num);
                    point.add_field(k, InfluxValue::Integer(num.as_i64().unwrap()));
                },
                _ => {
                    debug!("skipping: {:?}, {:?}", k, v);
                }

            }

        }
        Ok(point)

    }
}



impl Metric<MsgVpnResponse> for MsgVpnResponse {

    fn get(vpn_name: &str, subitem_name: &str, selector: &str, apiclient: &APIClient<HttpsConnector<HttpConnector>>, core: &mut Core) -> Result<MsgVpnResponse, &'static str> {

        let request = apiclient
            .default_api()
            .get_msg_vpn(vpn_name, getselect(selector))
            .and_then(|vpn| {
                println!("response: {:?}", vpn);
                futures::future::ok(vpn)
            });

        match core.run(request) {
            Ok(response) => {
                info!("{}",format!("{}", serde_yaml::to_string(&response.data().unwrap()).unwrap()));
                Ok(response)
            },
            Err(e) => {
                println!("get monitor error: {:?}", e);
                Err("fetch error")
            }
        }


    }

    fn create_metric(point: &str, item: &MsgVpnResponse, tags: HashMap<String, String>, influxdb_client: &mut InfluxClient) -> Point {

        let t = item.data().unwrap();

        let mut vpn_points = MsgVpnResponse::extract_data(item);

        for tag in tags {
            vpn_points.add_tag(tag.0, InfluxValue::String(tag.1));
        }


        vpn_points
            .add_timestamp(time::now().to_timespec().sec)
            .to_owned();

        info!("created point {:?}", vpn_points);

        vpn_points

    }

    fn extract_data(item: &MsgVpnResponse) -> Point {
        match serde_json::to_string(item.data().unwrap()) {
            Ok(s) => {
                match item.make_fields(s, "message-vpn") {
                    Ok(p) => {
                        p
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
}




