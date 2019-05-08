# solace-metrics

This is a exploratory program to determine the capabilities of Solace's SEMPv2 metrics interfaces.

## Goals

* Collect solace metrics over SEMPv2
* tag / dimension data
* persist to InfluxDB

## Running

```
RUST_LOG=info cargo run -- --output testdir --config solace.yaml --influxdb http://localhost:8086 --influxdb-user root --influxdb-pass root --influxdb-dbname smg vpn --message-vpn default 
```

## Testing

```
curl 'localhost:8086/query?pretty=true' --data-urlencode "db=smg" --data-urlencode "q=SELECT \"msg-spool-usage\" FROM \"message-vpn\""
```
