mod api;

use std::error::Error;
use std::fmt::{Debug, Formatter};
use api::{Response, ResponseError};
use api::zones::*;
use api::records::*;
use crate::api::SingleResult;

#[derive(Debug)]
pub struct ResultError(String);

#[derive(Debug)]
pub struct ResultResponseError {
    pub errors: Vec<ResponseError>
}

impl std::fmt::Display for ResultError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}

impl std::fmt::Display for ResultResponseError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let errors = &self.errors;
        let errors: Vec<String> = errors.into_iter().map(|x| format!("{}: {}", x.code, x.message)).collect();
        write!(f, "There is an error:\n{}", errors.join("\n"))
    }
}

impl Error for ResultError {}

impl Error for ResultResponseError {}

impl<T> From<Response<T>> for ResultResponseError {
    fn from(item: Response<T>) -> Self {
        let default = ResponseError {code: 0, message: "Unknown Error.".parse().unwrap() };
        ResultResponseError {
            errors: item.errors.unwrap_or(vec!(default))
        }
    }
}

impl<T> From<SingleResult<T>> for ResultResponseError {
    fn from(item: SingleResult<T>) -> Self {
        let default = ResponseError {code: 0, message: "Unknown Error.".parse().unwrap() };
        ResultResponseError {
            errors: item.errors.unwrap_or(vec!(default))
        }
    }
}

pub async fn get_zone_by_name(name: &str) -> Result<Zone, Box<dyn Error>> {
    let zones = api::zones::get_zones(Option::from(name)).await.unwrap();
    let zones = match zones.result {
        None => Vec::new(),
        Some(t) => t
    };
    match zones.into_iter().next() {
        None => Err(Box::new(ResultError(format!("No Zone found for name {}.", name)))),
        Some(zone) => Ok(zone)
    }
}

pub async fn get_all_records_by_name(zone_name: &str) -> Result<Response<Record>, Box<dyn Error>> {
    let zone = match get_zone_by_name(zone_name).await {
        Ok(zone) => zone,
        Err(e) => return Err(e)
    };
    Ok(get_all_records(zone.id).await?)
}


pub async fn delete_records_by_name(zone: &str, record_name: &str) -> Result<(), Box<dyn Error>> {
    let data = get_all_records_by_name(&zone)
        .await?
        .result
        .unwrap()
        .into_iter()
        .filter(|record| record.name == record_name);
    for record in data {
        delete_record(zone, record.id.as_str()).await;
    }
    Ok(())
}

pub async fn create_update_record(zone_name: &str, record_name: &str, value: &str, record_type: &str) -> Result<(), Box<dyn Error>> {
    let records = get_all_records_by_name(zone_name)
        .await?
        .result
        .unwrap()
        .into_iter()
        .filter(|record| record.name.starts_with(record_name))
        .collect::<Vec<_>>();
    let count = records.len();
    if count > 1 {
        delete_records_by_name(zone_name, record_name).await?;
    }
    if count == 1 {
        let mut records = records.iter();
        let record = records.next().unwrap();
        let zone = get_zone_by_name(zone_name).await?;
        update_record(
            record.id.as_str(),
            zone.id,
            Option::from(record_name),
            Option::from(record_type),
            Option::from(value),
            Option::None,
        ).await;
    } else {
        let zone = get_zone_by_name(zone_name).await?;
        create_record(record_name, record_type, value, zone.id).await;
    }
    Ok(())
}

