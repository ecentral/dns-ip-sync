mod api;

use api::zones::*;
use api::records::*;

pub async fn get_zone_by_name(name: &str) -> Zone {
    let zones = api::zones::get_zones(Option::from(name)).await.unwrap();
    zones.zones.into_iter().next().unwrap()
}

pub async fn get_all_records_by_name(zone_name: &str) -> Records {
    let zone = get_zone_by_name(zone_name).await;
    get_all_records(zone.id).await
}

pub async fn delete_records_by_name(zone: &str, record_name: &str) {
    let data = get_all_records_by_name(zone)
        .await
        .records
        .into_iter()
        .filter(|record| record.name == record_name);
    for record in data {
        delete_record(record.id.as_str()).await;
    }
}

pub async fn create_update_record(zone_name: &str, record_name: &str, value: &str, record_type: &str) {
    let count = get_all_records_by_name(zone_name)
        .await
        .records
        .into_iter()
        .filter(|record| record.name == record_name).count();
    if count > 1 {
        delete_records_by_name(zone_name, record_name).await;
    }
    if count == 1 {
        let mut data = get_all_records_by_name(zone_name).await.records.into_iter()
            .filter(|record| record.name == record_name);
        let record = data.next().unwrap();
        let zone = get_zone_by_name(zone_name).await;
        update_record(record.id.as_str(), record_name, record_type, value, zone.id).await;
        return;
    }
    let zone = get_zone_by_name(zone_name).await;
    create_record(record_name, record_type, value, zone.id).await;
}
