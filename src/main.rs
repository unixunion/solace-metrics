#[macro_use]
extern crate log;
extern crate env_logger;
extern crate influx_db_client;
extern crate itertools;


use std::error::Error;
use clap::{Arg, App, load_yaml};
use std::borrow::Cow;
use tokio_core::reactor::Core;
use hyper::client::HttpConnector;
use native_tls::{TlsConnector, Certificate};
use hyper::Client;
use solace_semp_client_monitor::apis::configuration::Configuration;
use solace_semp_client_monitor::apis::client::APIClient;
use influx_db_client::{Point, Points, Value, Precision};
use influx_db_client::Client as InfluxClient;
use crate::helpers::getselect;
use solace_semp_client_monitor::models::{MsgVpnResponse, MsgVpnsResponse};
use crate::metrics::Metric;
use crate::save::Save;
use std::collections::HashMap;

mod helpers;
mod clientconfig;
mod metrics;
mod save;
mod data;

fn main() -> Result<(), Box<Error>> {

    // initialize the logger
    env_logger::init();

    // load args.yaml
    let yaml = load_yaml!("args.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    // get the config file name
    let config_file_name = matches.value_of("config").unwrap();
    info!("config_file: {:?}", config_file_name);

    // selector
    let selector = matches.value_of("selector").unwrap_or("*");

    let influxdb_url = matches.value_of("influxdb").unwrap();
    let influxdb_user = matches.value_of("influxdb-user").unwrap();
    let influxdb_pass = matches.value_of("influxdb-user").unwrap();
    let influxdb_dbname = matches.value_of("influxdb-dbname").unwrap();

    let mut influxdb_client = InfluxClient::new(influxdb_url, influxdb_dbname);

    if influxdb_client.ping() != true {
        panic!("influxdb did not repluy to ping");
    }

    influxdb_client.create_database(influxdb_dbname).expect("already created");

    // save data
    let mut output_dir = "output";
    let mut write_fetch_files = false;
    if matches.is_present("output") {
        output_dir = matches.value_of("output").unwrap();
        write_fetch_files = true;
        info!("output_dir: {}", output_dir);
    }

    // future impl might use this.
    let mut cursor = Cow::Borrowed("");
    let mut select = "*";

    // default emoji for OK / Error logging
    let mut ok_emoji = Cow::Borrowed("👍");
    let mut err_emoji = Cow::Borrowed("❌");

    // configure the http client
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let mut http = HttpConnector::new(4, &handle);
    http.enforce_http(false);

    let mut tls = TlsConnector::builder()?;
    let mut sac = clientconfig::readconfig(config_file_name.to_owned());

    let mut metatags: HashMap<String, String> = HashMap::new();

    match sac {
        Ok(c) => {
            match c.certs {
                Some(certs) => {
                    for cert in certs.iter() {
                        info!("Adding certificate to chain");
                        let t: Certificate = Certificate::from_pem(cert.as_bytes())?;
                        tls.add_root_certificate(t);
                    }
                },
                None => info!("No certs")
            }
            match c.meta {

                Some(meta) => {
                    for kv in meta.iter() {
                        info!("meta data: {:?}", kv);
                        metatags.insert(kv.0.clone(), kv.1.clone());
                    }
                },
                None => info!("No meta")
            }
        },
        Err(e) => panic!()
    }

    let hyperclient = Client::configure()
        .connector(hyper_tls::HttpsConnector::from((http, tls.build()?))).build(&handle);


    // auth
    let auth = helpers::gencred("admin".to_owned(), "admin".to_owned());

    // the configuration for the APIClient
    let mut configuration = Configuration {
        base_path: "http://localhost:8080/SEMP/v2/config".to_owned(),
        user_agent: Some("Swagger-Codegen/2.10/rust".to_owned()),
        client: hyperclient,
        basic_auth: Some(auth),
        oauth_access_token: None,
        api_key: None,
    };


    let mut sac = clientconfig::readconfig(config_file_name.to_owned());
    match sac {
        Ok(sc) => {
            configuration.base_path = sc.host;
            let auth = helpers::gencred(sc.username, sc.password);
            configuration.basic_auth = Some(auth);
            ok_emoji = Cow::Owned(sc.ok_emoji);
            err_emoji = Cow::Owned(sc.err_emoji);
        },
        Err(e) => error!("error reading config: {}", e)
    }


    // the API Client from swagger spec
    let client = APIClient::new(configuration);

    // a vec to store points in
    let mut points: Vec<Point> = Vec::new();


    // VPN metrics
    if matches.is_present("vpn") {
        if let Some(sub_matches) = matches.subcommand_matches("vpn") {

            let x = MsgVpnResponse::get(
                sub_matches.value_of("message-vpn").unwrap(),
                "",
                selector,
                &client, &mut core);

            match x {
                Ok(vpn) => {
                    let p = MsgVpnResponse::create_metric("vpn-stats", &vpn, metatags, &mut influxdb_client);
                    points.push(p);

                    if write_fetch_files {
                        MsgVpnResponse::save(output_dir, &vpn);
                    }
                },
                Err(e) => {
                    error!("unable to persist metric")
                }
            }
        }
    }

    let mut pointbatch = Points::create_new(points);

    // if Precision is None, the default is second
    // Multiple write
    let _ = influxdb_client.write_points(pointbatch, Some(Precision::Seconds), None).unwrap();
    info!("wrote points");

    Ok(())
}

