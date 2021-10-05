mod api;

use local_ip_address::local_ip;
use std::env;
use api::zones::*;
use crate::api::records::*;
use seahorse::{App, Context, Flag, FlagType};
use tokio::runtime::Runtime;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let app = App::new(env!("CARGO_PKG_NAME"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage("cli [--ip=127.0.0.1]")
        .flag(
            Flag::new("ip", FlagType::String)
                .description("Use provided ip address instead of using ip of localhost")
        )
        .flag(
            Flag::new("zone", FlagType::String)
                .description("Use provided zone instead of env HETZNER_ZONE")
        )
        .flag(
            Flag::new("domain", FlagType::String)
                .description("Use provided domain instead of env HETZNER_DOMAIN")
        )
        .action(command);

    app.run(args);
    Ok(())
}

fn command(context: &Context) {
    let runtime = Runtime::new().expect("Init successful");
    runtime.block_on(async move {
        let my_local_ip = if context.string_flag("ip").is_ok() {
            context.string_flag("ip").unwrap()
        } else {
            local_ip().unwrap().to_string()
        };
        let zone = if context.string_flag("zone").is_ok() {
            context.string_flag("zone").unwrap()
        } else {
            env::var("HETZNER_ZONE").unwrap()
        };
        let domain = if context.string_flag("domain").is_ok() {
            context.string_flag("domain").unwrap()
        } else {
            env::var("HETZNER_DOMAIN").unwrap()
        };
        create_update_record(zone.as_str(),domain.as_str(), my_local_ip.as_str(), "A").await;
        let response = get_all_records_by_name(zone.as_str()).await;
        for record in response.records {
            if record.record_type == String::from("A") {
                println!("{:?}", record);
            }
        }
    });
}

async fn get_zone_by_name(name: &str) -> Zone {
    let zones = api::zones::get_zones(Option::from(name)).await.unwrap();
    zones.zones.into_iter().next().unwrap()
}

async fn get_all_records_by_name(zone_name: &str) -> Records {
    let zone = get_zone_by_name(zone_name).await;
    get_all_records(zone.id).await
}

async fn delete_records_by_name(zone: &str, record_name: &str) {
    let data = get_all_records_by_name(zone)
        .await
        .records
        .into_iter()
        .filter(|record| record.name == record_name);
    for record in data {
        delete_record(record.id.as_str()).await;
    }
}

async fn create_update_record(zone_name: &str, record_name: &str, value: &str, record_type: &str) {
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