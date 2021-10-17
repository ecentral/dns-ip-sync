use crate::{ResultResponseError};
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
pub async fn get_zones(name: Option<&str>) -> Result<Response<Zone>, Box<dyn std::error::Error>> {
    let mut client = cloudflare_client("zones", Method::GET);
    if name.is_some() {
        client = client.query(&[("name", name.unwrap())]);
    }
    let response = client.send().await?;
    let response = response.json::<Response<Zone>>().await?;
    if response.success {
        Ok(response)
    } else {
        let error = ResultResponseError::from(response);
        Result::Err(Box::from(error))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::mock;
    use tokio::runtime::Runtime;
    use std::env;

    #[test]
    fn test_get_zones_successfully() {
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
            assert!(zones.result.is_some());
            let zones = zones.result.unwrap();
            assert_eq!(1, zones.len());
            assert_eq!("example.com", zones[0].name.as_str());
        });
    }

    #[test]
    fn test_get_zones_status_is_false() {
        env::set_var("CLOUDFLARE_KEY", "");
        env::set_var("CLOUDFLARE_EMAIL", "");
        let _m = mock("GET", "/zones")
            .with_header("content-type", "application/json")
            .with_body(r#"
            {
                "success": false,
                "errors": [
                    {"code": 1074, "message": "Could not find a valid zone."}
                ]
            }
            "#)
            .create();
        let runtime = Runtime::new().expect("Init successful");
        runtime.block_on(async move {
            let result = get_zones(Option::None).await;
            assert!(result.is_err());
            let error = result.err();
            assert!(error.is_some());
            let error: Box<ResultResponseError> = error.unwrap().downcast().unwrap();
            assert_eq!(1, error.errors.len());
            assert_eq!("There is an error:\n1074: Could not find a valid zone.", format!("{}", error));
        });
    }
}