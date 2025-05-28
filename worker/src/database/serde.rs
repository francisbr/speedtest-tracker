use crate::database::bindings::Bandwidth;
use crate::database::bindings::Interface;
use crate::database::bindings::Latency;
use crate::database::bindings::Ping;
use crate::database::bindings::Server;
use crate::database::bindings::SpeedtestResult;
use serde::Deserialize;
use spacetimedb_sdk::Timestamp;

impl<'de> Deserialize<'de> for SpeedtestResult {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let r = deserialization_temp::Speedtest::deserialize(deserializer)?;

        Ok(Self {
            id: r.metadata.id.to_string(),
            url: r.metadata.url,
            timestamp: Timestamp::from_micros_since_unix_epoch(r.timestamp.timestamp_micros()),
            persisted: r.metadata.persisted,
            server: Server {
                id: r.server.id,
                host: r.server.host,
                port: r.server.port,
                name: r.server.name,
                location: r.server.location,
                country: r.server.country,
                ip: r.server.ip,
            },
            upload: Bandwidth {
                bandwidth: r.upload.bandwidth,
                bytes: r.upload.bytes,
                elapsed: r.upload.elapsed,
                latency: Latency {
                    iqm: r.upload.latency.iqm,
                    low: r.upload.latency.low,
                    high: r.upload.latency.high,
                    jitter: r.upload.latency.jitter,
                },
            },
            download: Bandwidth {
                bandwidth: r.download.bandwidth,
                bytes: r.download.bytes,
                elapsed: r.download.elapsed,
                latency: Latency {
                    iqm: r.download.latency.iqm,
                    low: r.download.latency.low,
                    high: r.download.latency.high,
                    jitter: r.download.latency.jitter,
                },
            },
            ping: Ping {
                jitter: r.ping.jitter,
                latency: r.ping.latency,
                low: r.ping.low,
                high: r.ping.high,
            },
            interface: Interface {
                internal_ip: r.interface.internal_ip,
                name: r.interface.name,
                mac_addr: r.interface.mac_addr,
                is_vpn: r.interface.is_vpn,
                external_ip: r.interface.external_ip,
            },
            packet_loss: r.packet_loss,
            isp: r.isp,
        })
    }
}

mod deserialization_temp {
    use chrono::DateTime;
    use chrono::Utc;
    use serde::Deserialize;
    use uuid::Uuid;

    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Speedtest {
        pub timestamp: DateTime<Utc>,
        #[serde(rename = "result")]
        pub metadata: Metadata,
        pub server: Server,
        pub upload: Bandwidth,
        pub download: Bandwidth,
        pub ping: Ping,
        pub interface: Interface,
        pub packet_loss: Option<u64>,
        pub isp: String,
    }

    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Metadata {
        pub id: Uuid,
        pub url: String,
        pub persisted: bool,
    }

    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Server {
        pub id: u64,
        pub host: String,
        pub port: u16,
        pub name: String,
        pub location: String,
        pub country: String,
        pub ip: String,
    }

    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Bandwidth {
        pub bandwidth: u64,
        pub bytes: u64,
        pub elapsed: u64,
        pub latency: Latency,
    }

    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Latency {
        pub iqm: f64,
        pub low: f64,
        pub high: f64,
        pub jitter: f64,
    }

    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Ping {
        pub jitter: f64,
        pub latency: f64,
        pub low: f64,
        pub high: f64,
    }

    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Interface {
        pub internal_ip: String,
        pub name: String,
        pub mac_addr: String,
        pub is_vpn: bool,
        pub external_ip: String,
    }
}

#[cfg(test)]
mod tests {
    use crate::database::bindings::SpeedtestResult;

    #[test]
    fn simple() {
        let result = serde_json::from_str::<SpeedtestResult>(
            r#"{"type":"result","timestamp":"2025-05-27T20:49:45Z","ping":{"jitter":0.756,"latency":3.274,"low":2.953,"high":3.993},"download":{"bandwidth":68261276,"bytes":368342740,"elapsed":5408,"latency":{"iqm":15.132,"low":4.042,"high":17.440,"jitter":0.830}},"upload":{"bandwidth":68868789,"bytes":500375582,"elapsed":7301,"latency":{"iqm":11.247,"low":8.936,"high":14.776,"jitter":0.780}},"packetLoss":0,"isp":"EBOX","interface":{"internalIp":"192.168.50.16","name":"","macAddr":"10:FF:E0:6C:8E:BB","isVpn":false,"externalIp":"107.159.240.113"},"server":{"id":38690,"host":"speedtest.altimatel.com","port":8080,"name":"Altima Telecom","location":"Montréal, QC","country":"Canada","ip":"65.38.64.100"},"result":{"id":"7e12b728-34c4-4eaf-bedd-d493d6d371b8","url":"https://www.speedtest.net/result/c/7e12b728-34c4-4eaf-bedd-d493d6d371b8","persisted":true}}"#,
        );

        assert!(result.is_ok());
    }
}
