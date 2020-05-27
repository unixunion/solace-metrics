
extern crate influx_db_client;

use influx_db_client::{Point, Points, Precision};
use influx_db_client::Client as InfluxClient;
use influx_db_client::Value as InfluxValue;
use solace_semp_client_monitor::models::{MsgVpnResponse, MsgVpnsResponse, MsgVpnQueueResponse, MsgVpnQueuesResponse};
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
use crate::metric::Metric;


mod test {

}


pub struct MsgVpnQueueReq{
    pub msg_vpn_name: String,
    pub queue_name: String,
    pub selectv: String
}


impl Metric<MsgVpnQueueResponse> for MsgVpnQueueReq {
    //queue_name: &str, subitem_name: &str, selector: &str
    fn get(&self, apiclient: &APIClient<HttpsConnector<HttpConnector>>, core: &mut Core) -> Result<MsgVpnQueueResponse, &'static str> {

        let selector = self.selectv.as_ref();

        let request = apiclient
            .default_api()
            .get_msg_vpn_queue(self.msg_vpn_name.as_ref(), self.queue_name.as_ref(), getselect(selector))
            .and_then(|queue| {
                println!("response: {:?}", queue);
                futures::future::ok(queue)
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

    fn create_metric(point: &str, item: &MsgVpnQueueResponse, tags: HashMap<String, String>, influxdb_client: &mut InfluxClient) -> Vec<Point> {

        let t = item.data().unwrap();

        let mut vpn_points = MsgVpnQueueReq::extract_data(item);

        for tag in tags {
            for mut v in &mut vpn_points {
                v.add_tag(&tag.0, InfluxValue::String(tag.1.clone()));
                v.add_timestamp(time::now().to_timespec().sec);
            }
        }

        info!("created point {:?}", vpn_points);

        vpn_points

    }

    fn extract_data(item: &MsgVpnQueueResponse) -> Vec<Point> {
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
        unimplemented!()
    }
}




