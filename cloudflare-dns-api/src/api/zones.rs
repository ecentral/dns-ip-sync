use super::*;

#[derive(Deserialize, Debug)]
pub struct Owner {
    pub id: Option<String>,
    pub email: Option<String>,
    #[serde(rename="type")]
    pub owner_type: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Account {
    pub id: String,
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct Zone {
    pub id: String,
    pub name: String,
    pub development_mode: i32,
    pub original_registrar: Option<String>,
    pub original_dnshost: Option<String>,
    pub original_name_servers: Option<Vec<String>>,
    pub name_servers: Vec<String>,
    pub permissions: Vec<String>,
    pub owner: Owner,
    pub account: Account,
    pub created_on: String,
    pub modified_on: String,
    pub activated_on: String,
    #[serde(rename="type")]
    pub zone_type: String,
    pub status: String,
    pub paused: bool,
}

/// # Links
/// [see cloudflare documentation](https://api.cloudflare.com/#zone-zone-details)
///
/// # Arguments
/// * `name` - filter by zone-name, e.g. example.com
///
pub async fn get_zones(name: Option<&str>) -> Result<Response<Zone>, Box<dyn std::error::Error>> {
    let mut client = cloudflare_client("zones", Method::GET);
    if name.is_some() {
        client = client.query(&[("name", name.unwrap())]);
    }
    Ok(client.send().await?.json::<Response<Zone>>().await?)
}