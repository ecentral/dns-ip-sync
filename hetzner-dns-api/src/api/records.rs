use super::*;
use reqwest::Method;

#[derive(Deserialize, Debug)]
pub struct Record {
    #[serde(rename="type")]
    pub record_type: String,
    pub id: String,
    pub created: String,
    pub modified: String,
    pub zone_id: String,
    pub name: String,
    pub value: String,
}

#[derive(Deserialize, Serialize)]
pub struct RecordCreate {
    #[serde(rename="type")]
    pub record_type: String,
    pub zone_id: String,
    pub name: String,
    pub value: String,
}

#[derive(Deserialize, Debug)]
pub struct Records {
    pub records: Vec<Record>
}

pub async fn create_record(name: &str, record_type: &str, value: &str, zone_id: String) -> String {
    let new_record = RecordCreate {
        record_type: String::from(record_type),
        value: String::from(value),
        name: String::from(name),
        zone_id: String::from(zone_id),
    };
    let client = hetzner_client("records", Method::POST)
        .json(&new_record);
    client.send().await.unwrap().text().await.unwrap()
}

pub async fn delete_record(record_id: &str) {
    let url = format!("{}/{}", "records", String::from(record_id));
    let client = hetzner_client(&*url, Method::DELETE);
    client.send().await.unwrap().status();
}

pub async fn get_all_records(zone_id: String) -> Records {
    let client = hetzner_client("records", Method::GET)
        .query(&[("zone_id", zone_id)]);
    client.send().await.unwrap().json::<Records>().await.unwrap()
}

pub async fn update_record(id: &str, name: &str, record_type: &str, value: &str, zone_id: String) -> String {
    let url = format!("{}/{}", "records", id);
    let new_record = RecordCreate {
        record_type: String::from(record_type),
        value: String::from(value),
        name: String::from(name),
        zone_id: String::from(zone_id),
    };
    let client = hetzner_client(url.as_str(), Method::PUT)
        .json(&new_record);
    client.send().await.unwrap().text().await.unwrap()
}