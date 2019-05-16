# solace-metrics

This is a exploratory program to determine the capabilities of Solace's SEMPv2 metrics interfaces.

## Goals

* Collect solace metrics over SEMPv2
* tag / dimension data
* persist to InfluxDB

## Status

POC to query some [message-vpn](src/metrics.rs) metrics, and persist them to influx.

## Todo

All other metrics, pending Solace issue with OpenAPI spec.

## Building

```
cargo build --release
```

## Running

Runs once, writes metrics to influx, and exits.

```
solace-monitor 0.0.1
Kegan Holtzhausen <kegan.holtzhausen@kindredgroup.com>
SEMPv2 version 9.1.0.77 solace monitoring tool, see https://github.com/unixunion/solace-monitor for src and examples

USAGE:
    solace-monitor [OPTIONS] --config <CONFIG> --influxdb <influxdb> --influxdb-dbname <influxdb-dbname> --influxdb-pass <influxdb-pass> --influxdb-user <influxdb-user> [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --config <CONFIG>                      Sets the solace config file
        --influxdb <influxdb>                  influxdb url e.g http://127.0.0.1:8086
        --influxdb-dbname <influxdb-dbname>    the db name
        --influxdb-pass <influxdb-pass>        influxdb password
        --influxdb-user <influxdb-user>        influxdb user
        --output <output>                      output dir
        --selector <selector>                  selector, default "*"

SUBCOMMANDS:
    help    Prints this message or the help of the given subcommand(s)
    vpn     vpn metrics
```

### VPN metrics

```
solace-monitor-vpn 9.1.0.77
Kegan Holtzhausen <kegan.holtzhausen@kindredgroup.com>
vpn metrics

USAGE:
    solace-monitor vpn --message-vpn <message-vpn>...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --message-vpn <message-vpn>...    vpn(s) to fetch
```

### Example

```
RUST_LOG=info solace-monitor \
    --config solace.yaml \
    --influxdb http://localhost:8086 \
    --influxdb-user root \
    --influxdb-pass root \
    --influxdb-dbname smg \
    vpn --message-vpn default other_vpn another_vpn
```

## Running Dev

```
docker-compose up -d

solace-monitor [--output testdir] --config solace.yaml --influxdb http://localhost:8086 --influxdb-user root --influxdb-pass root --influxdb-dbname smg vpn --message-vpn default anothervpn anotherone andanotherone```

curl 'localhost:8086/query?pretty=true' --data-urlencode "db=smg" --data-urlencode "q=SELECT \"rate_rxMsgRate\" FROM \"message-vpn\""
```
