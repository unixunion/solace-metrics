# solace-metrics

This is a exploratory program to determine the capabilities of Solace's SEMPv2 metrics interfaces.

## Goals

* Collect solace metrics over SEMPv2
* tag / dimension data
* persist to InfluxDB

## Status

POC to query some [message-vpn](src/metrics.rs) metrics, and persist them to influx.

## Running Dev

```
docker-compose up -d

solace-monitor [--output testdir] --config solace.yaml --influxdb http://localhost:8086 --influxdb-user root --influxdb-pass root --influxdb-dbname smg vpn --message-vpn default anothervpn anotherone andanotherone```

curl 'localhost:8086/query?pretty=true' --data-urlencode "db=smg" --data-urlencode "q=SELECT \"rate_rxMsgRate\" FROM \"message-vpn\""
```
