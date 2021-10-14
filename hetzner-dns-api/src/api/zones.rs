use super::*;
use reqwest::Method;

#[derive(Deserialize, Serialize, Debug)]
pub struct Zone {
    pub id: String,
    pub created: String,
    pub modified: String,
    pub legacy_dns_host: String,
    pub legacy_ns: Vec<String>,
    pub name: String,
    pub ns: Vec<String>,
    pub owner: String,
    pub paused: bool,
    pub permission: String,
    pub project: String,
    pub registrar: String,
    pub status: String,
    pub ttl: i32,
    pub verified: String,
    pub records_count: i32,
    pub is_secondary_dns: bool,
    pub txt_verification: TxtVerification
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TxtVerification {
    pub name: String,
    pub token: String,
}

#[derive(Deserialize, Debug)]
pub struct Zones {
    pub zones: Vec<Zone>,
}

#[derive(Deserialize, Debug)]
pub struct ZonesResponse {
    pub zones: Option<Vec<Zone>>,
}

#[derive(Deserialize, Debug)]
pub struct ZoneResponse {
    pub zone: Option<Zone>,
}

pub async fn get_zones(name: Option<&str>) -> Result<ZonesResponse, Box<dyn std::error::Error>> {
    let mut client = hetzner_client("zones", Method::GET);
    if name.is_some() {
        client = client.query(&[("name", name.unwrap())]);
    }
    Ok(client.send().await?.json::<ZonesResponse>().await?)
}

pub async fn get_zone(zone_id: &str) -> Result<ZoneResponse, Box<dyn std::error::Error>> {
    let url = format!("{}/{}", "zones", String::from(zone_id));
    let client = hetzner_client(&*url, Method::GET);

    Ok(client.send().await?.json::<ZoneResponse>().await?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::mock;
    use tokio::runtime::Runtime;
    use std::env;

    #[test]
    fn test_get_zones() {
        env::set_var("HETZNER_TOKEN", "");
        let _m = mock("GET", "/zones")
            .with_header("content-type", "application/json")
            .with_body(format!("{{ \"zones\": [{}] }}", get_zone_mock("2", "zone2.test")))
            .create();
        let runtime = Runtime::new().expect("Init successful");
        runtime.block_on(async move {
            let zones = get_zones(Option::None).await.unwrap();
            assert!(zones.zones.is_some());
            let zones = zones.zones.unwrap();
            assert_eq!(1, zones.len());
            assert_eq!("zone2.test", zones[0].name.as_str());
        });
    }

    #[test]
    fn test_get_zone() {
        env::set_var("HETZNER_TOKEN", "");
        let _m = mock("GET", "/zones/23")
            .with_header("content-type", "application/json")
            .with_body(format!("{{ \"zone\": {} }}", get_zone_mock("23", "zone23.test")))
            .create();
        let runtime = Runtime::new().expect("Init successful");
        runtime.block_on(async move {
            let zone = get_zone("23" ).await.unwrap();
            assert!(zone.zone.is_some());
            let zone = zone.zone.unwrap();
            assert_eq!("zone23.test", zone.name.as_str());
        });
    }

    fn get_zone_mock(zone_id: &str, zone_name: &str) -> String {
        let zone = Zone {
            id: String::from(zone_id),
            created: "".to_string(),
            modified: "".to_string(),
            legacy_dns_host: "".to_string(),
            legacy_ns: vec![],
            name: zone_name.to_string(),
            ns: vec![],
            owner: "".to_string(),
            paused: false,
            permission: "".to_string(),
            project: "".to_string(),
            registrar: "".to_string(),
            status: "".to_string(),
            ttl: 0,
            verified: "".to_string(),
            records_count: 0,
            is_secondary_dns: false,
            txt_verification: TxtVerification { name: "".to_string(), token: "".to_string() }
        };
        serde_json::to_value(&zone).unwrap().to_string()
    }
}
