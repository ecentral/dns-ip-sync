use super::*;
use reqwest::Method;

#[derive(Deserialize, Debug)]
pub struct Zone {
    pub id: String,
    pub name: String,
    pub ttl: i32,
    pub registrar: String,
    pub legacy_dns_host: String,
    pub legacy_ns: Vec<String>,
    pub ns: Vec<String>,
    pub created: String,
    pub verified: String,
    pub modified: String,
    pub project: String,
    pub owner: String,
    pub permission: String,
    pub status: String,
    pub paused: bool,
    pub is_secondary_dns: bool,
    pub records_count: i32,
}

#[derive(Deserialize, Debug)]
pub struct Zones {
    pub zones: Vec<Zone>
}

pub async fn get_zones(name: Option<&str>) -> Result<Zones, Box<dyn std::error::Error>> {
    let mut client = hetzner_client("https://dns.hetzner.com/api/v1/zones", Method::GET);
    if name.is_some() {
        client = client.query(&[("name", name.unwrap())]);
    }
    Ok(client.send().await?.json::<Zones>().await?)
}