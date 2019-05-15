
extern crate influx_db_client;

use influx_db_client::{Point, Points, Precision};
use influx_db_client::Client as InfluxClient;
use influx_db_client::Value as InfluxValue;
use solace_semp_client_monitor::models::{MsgVpnResponse, MsgVpnsResponse};
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
use crate::data::Data;

mod test {

}


pub trait Metric<T> {

    fn get(item_name: &str, subitem_name: &str, selector: &str, apiclient: &APIClient<HttpsConnector<HttpConnector>>, core: &mut Core) -> Result<T, &'static str>;
    fn create_metric(point: &str, item: &T, tags: HashMap<String, String>, influxdb_client: &mut InfluxClient) -> Point;
    fn extract_data(item: &T) -> Point;

    fn make_fields(&self, data: String, measurement_name: &str) -> Result<Point, &'static str> {

        let mut point = Point::new(measurement_name);

        info!("string: {:?}", data);
        let t: HashMap<String, Value> = serde_json::from_str(&data).unwrap();
        for (k,v) in t.into_iter() {
            info!("key: {:?} value: {:?}", k, v);
            match v.as_object() {
                Some(o) => {
                    let x = format!("{:?}", serde_json::to_string(&v).unwrap());
                    info!("object: {:?} {:?}", k, x);
                    self.make_fields(x, measurement_name);
                }
                None => {
                    match v.as_i64() {
                        Some(v1) => {
                            info!("{:?} {:?}", k, v1);
                            point.add_field(k, InfluxValue::Integer(v1));
                        },
                        None => {
                            error!("skipping: {:?}, {:?}", k, v);
                        }
                    }
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

