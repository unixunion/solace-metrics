
extern crate influx_db_client;

use influx_db_client::{Point, Points, Precision};
use influx_db_client::Client as InfluxClient;
use influx_db_client::Value as InfluxValue;
use std::collections::HashMap;
use solace_semp_client_monitor::apis::client::APIClient;
use hyper_tls::HttpsConnector;
use hyper::client::HttpConnector;
use crate::helpers::{getselect, getwhere};
use tokio_core::reactor::Core;
use futures::future::Future;
use std::time::{SystemTime, UNIX_EPOCH};
use itertools::{Itertools, iterate};
use serde_json::Value;
use serde::Serialize;
use crate::metric::Metric;
use solace_semp_client_monitor::models::{MsgVpnClientConnection, MsgVpnClientConnectionsResponse, MsgVpnResponse, MsgVpnClientResponse, MsgVpnClientsResponse};
use crate::helpers;


mod test {

}

pub struct MsgVpnClientReq {
    pub vpn_name: String,
    pub client_name: String,
    pub count: i32,
    pub cursor: String,
    pub selector: String
}


impl Metric<MsgVpnClientsResponse> for MsgVpnClientReq {

    fn get(&self, apiclient: &APIClient<HttpsConnector<HttpConnector>>, core: &mut Core) -> Result<MsgVpnClientsResponse, &'static str> {

        let (wherev, selectv) = helpers::getwhere("clientName", self.client_name.as_ref(), self.selector.as_ref());

        let request = apiclient
            .client_api()
            .get_msg_vpn_clients(self.vpn_name.as_ref(), self.count, &self.cursor.as_str(), wherev, selectv)
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

    fn create_metric(point: &str, item: &MsgVpnClientsResponse, tags: HashMap<String, String>, influxdb_client: &mut InfluxClient) -> Vec<Point> {

        let t = item.data().unwrap();

        let mut vpn_points = MsgVpnClientsResponse::extract_data(item);

        for tag in tags {
            for mut i in &mut vpn_points {
                i.add_tag(&tag.0, InfluxValue::String(tag.1.clone()));
                i.add_timestamp(time::now().to_timespec().sec);
            }
        }


        info!("created point {:?}", vpn_points);

        vpn_points

    }

    fn extract_data(item: &MsgVpnClientsResponse) -> Vec<Point> {

        let mut vecp = vec![];

        match (item.data()) {
            Some(clients) => {
                for client in clients {
                    match serde_json::to_string(client) {
                        Ok(c) => {
                            let point = match item.make_fields(c, "client") {
                                Ok(p) => {
                                    vecp.push(p);
                                }
                                _ => {
                                    unimplemented!()
                                }
                            };
                        }
                        _ => {
                            unimplemented!()
                        }
                    }
                }
            }
            _ => {
                unimplemented!()
            }
        }

        vecp

    }

    fn make_fields(&self, data: String, measurement_name: &str) -> Result<Point, &'static str> {
        unimplemented!()
    }
}




