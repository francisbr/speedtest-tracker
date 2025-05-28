use spacetimedb::SpacetimeType;

#[derive(SpacetimeType, Default, Debug)]
pub struct Server {
    id: u64,
    host: String,
    port: u16,
    name: String,
    location: String,
    country: String,
    ip: String,
}

#[derive(SpacetimeType, Default, Debug)]
pub struct Bandwidth {
    bandwidth: u64,
    bytes: u64,
    elapsed: u64,
    latency: Latency,
}

#[derive(SpacetimeType, Default, Debug)]
pub struct Latency {
    iqm: f64,
    low: f64,
    high: f64,
    jitter: f64,
}

#[derive(SpacetimeType, Default, Debug)]
pub struct Ping {
    jitter: f64,
    latency: f64,
    low: f64,
    high: f64,
}

#[derive(SpacetimeType, Default, Debug)]
pub struct Interface {
    internal_ip: String,
    name: String,
    mac_addr: String,
    is_vpn: bool,
    external_ip: String,
}
