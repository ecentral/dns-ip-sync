use std::env;
use local_ip_address::local_ip;
use seahorse::{App, Context, Flag, FlagType};
use tokio::runtime::Runtime;
use tokio::join;
use hetzner_dns_api::{create_update_record, get_all_records_by_name};
use cloudflare_dns_api::{test};

/**

todo:
 - add support for toml to sync ip to multiple domains at the same time

 **/

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let app = App::new(env!("CARGO_PKG_NAME"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage("cli [--cloudflare-dns] [--ip=127.0.0.1] [--zone=your-zone] [--domain=your-domain]")
        .flag(
            Flag::new("cloudflare-dns", FlagType::Bool)
                .description("Use cloudflare instead of hetzner")
        ).
        flag(
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
        if context.bool_flag("cloudflare-dns") {
            let cloudflare = cloudflare_dns_api::test();
            // todo: this will allow running multiple futures
            join!(cloudflare);
            return;
        }
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
        let response = create_update_record(zone.as_str(), domain.as_str(), my_local_ip.as_str(), "A")
            .then(get_all_records_by_name(zone.as_str())).await;
        for record in response.records {
            if record.record_type == String::from("A") {
                println!("{:?}", record);
            }
        }
    });
}
