use aws_config::default_provider::credentials::DefaultCredentialsChain;
use aws_sdk_codecommit::config::Region;
use aws_sdk_codecommit::{Client, Config};
use log::debug;

pub async fn initialize_client(region: &str, profile: &str) -> Client {
    let region = Region::new(region.to_owned());

    let credentials_provider = DefaultCredentialsChain::builder()
        .region(region.clone())
        .profile_name(profile)
        .build()
        .await;
    let config = Config::builder()
        .credentials_provider(credentials_provider)
        .region(region.clone())
        .build();

    Client::from_conf(config)
}
