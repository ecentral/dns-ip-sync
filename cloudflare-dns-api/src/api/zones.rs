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
    pub owner: Option<Owner>,
    pub account: Option<Account>,
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
pub async fn get_zones(name: Option<&str>) -> Result<ZonesResponse<Zone>, Box<dyn std::error::Error>> {
    let mut client = cloudflare_client("zones", Method::GET);
    if name.is_some() {
        client = client.query(&[("name", name.unwrap())]);
    }
    Ok(client.send().await?.json::<ZonesResponse<Zone>>().await?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::mock;
    use tokio::runtime::Runtime;
    use std::env;

    #[test]
    fn test_get_zones() {
        env::set_var("CLOUDFLARE_KEY", "");
        env::set_var("CLOUDFLARE_EMAIL", "");
        let _m = mock("GET", "/zones")
            .with_header("content-type", "application/json")
            .with_body(r#"
            {
                "success": true,
                "result": [
                    {
                        "id": "1-1-1-1",
                        "name": "example.com",
                        "development_mode": 1,
                        "name_servers": ["example.ns.com"],
                        "permissions": ["read", "write"],
                        "created_on": "20.05.2021",
                        "modified_on": "20.05.2021",
                        "activated_on": "20.05.2021",
                        "type": "full",
                        "status": "active",
                        "paused": false
                    }
                ]
            }
            "#)
            .create();
        let runtime = Runtime::new().expect("Init successful");
        runtime.block_on(async move {
            let zones = get_zones(Option::None).await.unwrap();
            assert!(zones.success);
            assert_eq!(1, zones.result.len());
            assert_eq!("example.com", zones.result[0].name.as_str());
        });
    }
}