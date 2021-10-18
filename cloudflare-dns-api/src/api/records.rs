use reqwest::StatusCode;
use crate::{ResultResponseError};
use super::*;

#[derive(Deserialize, Debug)]
pub struct Record {
    pub id: String,
    #[serde(rename = "type")]
    pub record_type: String,
    pub name: String,
    pub content: String,
    pub proxiable: Option<bool>,
    pub proxied: Option<bool>,
    pub ttl: i16,
    pub locked: bool,
    pub zone_id: String,
    pub zone_name: String,
    pub created_on: String,
    pub modified_on: String,
}

#[derive(Deserialize, Serialize)]
pub struct RecordCreate {
    #[serde(rename = "type")]
    pub record_type: Option<String>,
    pub name: Option<String>,
    pub content: Option<String>,
    pub ttl: Option<i16>,
}

pub async fn get_all_records(zone_id: String) -> ResultResponse<Record> {
    let url = format!("zones/{}/dns_records", zone_id);
    let client = cloudflare_client(url.as_str(), Method::GET);
    let response = client.send().await?;
    let response = response.json::<Response<Record>>().await?;
    if response.success {
        Ok(response)
    } else {
        let error = ResultResponseError::from(response);
        Result::Err(Box::from(error))
    }
}

pub async fn create_record(name: &str, record_type: &str, value: &str, zone_id: String) -> SingleResultResponse<Record> {
    let url = format!("zones/{}/dns_records", zone_id);
    let new_record = RecordCreate {
        record_type: Option::Some(String::from(record_type)),
        content: Option::Some(String::from(value)),
        name: Option::Some(String::from(name)),
        ttl: Option::None,
    };
    let client = cloudflare_client(url.as_str(), Method::POST)
        .json(&new_record);
    let result = client.send().await?.json::<SingleResult<Record>>().await?;
    if result.success {
        Ok(result)
    } else {
        let error = ResultResponseError::from(result);
        Result::Err(Box::from(error))
    }
}

pub async fn delete_record(zone_id: &str, record_id: &str) -> OnlyResultResponse<OnlyId> {
    let url = format!("zones/{}/dns_records/{}", zone_id, record_id);
    let client = cloudflare_client(&*url, Method::DELETE);
    let result = client.send().await?.json::<OnlyResult<OnlyId>>().await?;
    // todo: documentation doesnt say response types on error, analyse that
    Ok(result)
}


pub async fn update_record(record_id: &str, zone_id: String, name: Option<&str>, record_type: Option<&str>, value: Option<&str>, ttl: Option<i16>) -> SingleResultResponse<Record> {
    let url = format!("zones/{}/dns_records/{}", zone_id, record_id);
    let update_record = RecordCreate {
        record_type: match record_type.is_some() {
            true => Option::Some(String::from(record_type.unwrap())),
            _ => Option::None,
        },
        content: match value.is_some() {
            true => Option::Some(String::from(value.unwrap())),
            _ => Option::None,
        },
        name: match name.is_some() {
            true => Option::Some(String::from(name.unwrap())),
            _ => Option::None,
        },
        ttl,
    };
    let method = if name.is_some() && record_type.is_some() && value.is_some() && ttl.is_some() {
        Method::PUT
    } else {
        Method::PATCH
    };
    let client = cloudflare_client(url.as_str(), method).json(&update_record);
    let result = client.send().await?.json::<SingleResult<Record>>().await?;
    if result.success {
        Ok(result)
    } else {
        let error = ResultResponseError::from(result);
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
    fn test_get_all_records() {
        env::set_var("CLOUDFLARE_KEY", "");
        env::set_var("CLOUDFLARE_EMAIL", "");
        let _m = mock("GET", "/zones/1-1-1-1/dns_records")
            .with_header("content-type", "application/json")
            .with_body(r#"{
                "success": true,
                "result": [
                    {
                        "id": "1-1-1-1",
                        "type": "test",
                        "name": "test",
                        "content": "example.com",
                        "proxiable": false,
                        "proxied": false,
                        "ttl": 3600,
                        "locked": false,
                        "zone_id": "1-1-1-1",
                        "zone_name": "example.com",
                        "created_on": "20.05.2021",
                        "modified_on": "20.05.2021"
                    }
                ]
            }"#)
            .create();
        let runtime = Runtime::new().expect("Init successful");
        runtime.block_on(async move {
            let zone_id = String::from("1-1-1-1");
            let records = get_all_records(zone_id).await;
            let record = Record {
                id: String::from("1-1-1-1"),
                record_type: "test".to_string(),
                name: "test".to_string(),
                content: "example.com".to_string(),
                proxiable: Some(false),
                proxied: Some(false),
                ttl: 3600,
                locked: false,
                zone_id: "1-1-1-1".to_string(),
                zone_name: "example.com".to_string(),
                created_on: "20.05.2021".to_string(),
                modified_on: "20.05.2021".to_string()
            };
            assert!(records.is_ok());
            let records = records.unwrap();
            assert!(records.result.is_some());
            let records = records.result.unwrap();
            assert_eq!(records.len(), 1);
            let json_record = records.first().unwrap();
            assert_eq!(json_record.id, record.id);
            assert_eq!(json_record.content, record.content);
            assert_eq!(json_record.created_on, record.created_on);
            assert_eq!(json_record.ttl, record.ttl);
            assert_eq!(json_record.record_type, record.record_type);
        });
    }

    #[test]
    fn test_get_all_records_with_error_data() {
        env::set_var("CLOUDFLARE_KEY", "");
        env::set_var("CLOUDFLARE_EMAIL", "");
        let _m = mock("GET", "/zones/1-1-1-1/dns_records")
            .with_header("content-type", "application/json")
            .with_body(r#"{
                "success": false,
                "errors": [{"code": 1, "message": "Error happened!"}]
            }"#)
            .create();
        let runtime = Runtime::new().expect("Init successful");
        runtime.block_on(async move {
            let zone_id = String::from("1-1-1-1");
            let records = get_all_records(zone_id).await;

            assert!(records.is_err());
            let error = records.err();
            assert!(error.is_some());
            let error: Box<ResultResponseError> = error.unwrap().downcast().unwrap();
            assert_eq!(1, error.errors.len());
            assert_eq!("There is an error:\n1: Error happened!", format!("{}", error));
        });
    }
}