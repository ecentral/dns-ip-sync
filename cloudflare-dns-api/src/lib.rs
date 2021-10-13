use crate::api::zones::get_zones;

mod api;

pub async fn test() {
    let result = get_zones(Option::None).await;
    println!("{:?}", result.unwrap());
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
