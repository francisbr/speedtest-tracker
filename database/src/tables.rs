use crate::types::Bandwidth;
use crate::types::Interface;
use crate::types::Ping;
use crate::types::Server;
use spacetimedb::table;
use spacetimedb::Timestamp;

#[derive(Debug)]
#[table(name = speedtest, public)]
pub struct SpeedtestResult {
    #[primary_key]
    pub id: String,
    pub url: String,
    #[index(btree)]
    pub timestamp: Timestamp,
    pub persisted: bool,

    pub server: Server,
    pub upload: Bandwidth,
    pub download: Bandwidth,
    pub ping: Ping,
    pub interface: Interface,

    pub packet_loss: Option<u64>,
    pub isp: String,
}
