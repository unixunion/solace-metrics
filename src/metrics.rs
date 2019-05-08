extern crate influx_db_client;

use influx_db_client::{Point, Points, Value, Precision};
use influx_db_client::Client as InfluxClient;
use solace_semp_client_monitor::models::{MsgVpnResponse, MsgVpnsResponse};
use std::collections::HashMap;
use solace_semp_client_monitor::apis::client::APIClient;
use hyper_tls::HttpsConnector;
use hyper::client::HttpConnector;
use crate::helpers::getselect;
use tokio_core::reactor::Core;
use futures::future::Future;
use std::time::{SystemTime, UNIX_EPOCH};

mod test {

}

pub trait Metric<T> {
    fn get(item_name: &str, subitem_name: &str, selector: &str, apiclient: &APIClient<HttpsConnector<HttpConnector>>, core: &mut Core) -> Result<T, &'static str>;
    fn create_metric(point: &str, item: &T, tags: HashMap<String, String>, influxdb_client: &mut InfluxClient) -> Point;
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
//                MsgVpnResponse::persist(output_dir, response);
            },
            Err(e) => {
                println!("get monitor error: {:?}", e);
                Err("fetch error")
            }
        }


    }

    fn create_metric(point: &str, item: &MsgVpnResponse, tags: HashMap<String, String>, influxdb_client: &mut InfluxClient) -> Point {

        let t = item.data().unwrap();

        let mut point1 = Point::new(point);

        for tag in tags {
            point1.add_tag(tag.0, Value::String(tag.1));
        }

        point1
            .add_tag("msg-vpn-name", Value::String(t.msg_vpn_name().cloned().unwrap()))
            .add_field("msg-spool-usage", Value::Integer(t.msg_spool_usage().cloned().unwrap()))
            .add_field("avg-rx-msg-rate", Value::Integer(*t.rate().cloned().unwrap().average_rx_msg_rate().unwrap()))
            .add_field("avg-tx-msg-rate", Value::Integer(*t.rate().cloned().unwrap().average_tx_msg_rate().unwrap()))
            .add_field("avg-rx-byte-rate", Value::Integer(*t.rate().cloned().unwrap().average_rx_byte_rate().unwrap()))
            .add_field("avg-tx-byte-rate", Value::Integer(*t.rate().cloned().unwrap().average_tx_byte_rate().unwrap()))

            .add_timestamp(time::now().to_timespec().sec)
            .to_owned();

        info!("created point {:?}", point1);
        point1

    }
}

