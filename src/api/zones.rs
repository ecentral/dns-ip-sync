use reqwest::Method;

use crate::api::{hetzner_client, Zones};

pub async fn get_zones(name: Option<&str>) -> Result<Zones, Box<dyn std::error::Error>> {
    let mut client = hetzner_client("https://dns.hetzner.com/api/v1/zones", Method::GET);
    if name.is_some() {
        client = client.query(&[("name", name.unwrap())]);
    }
    Ok(client.send().await?.json::<Zones>().await?)
}