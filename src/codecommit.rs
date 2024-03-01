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


pub async fn list_repositories(
    client: &Client,
    in_: &Vec<String>,
    out: &Vec<String>,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut repos = Vec::new();
    let mut repos_stream = client.list_repositories().into_paginator().send();
    while let Some(output) = repos_stream.next().await {
        for repo in output.unwrap().repositories.unwrap() {
            let repo_name = repo.repository_name.clone().expect("No repo name");
            if !out.iter().any(|x| repo_name.contains(x))
                && in_.iter().any(|x| repo_name.contains(x))
            {
                repos.push(repo_name);
            }
        }
    }
    debug!("Repositories: {:?}", repos);
    Ok(repos)
}

pub async fn list_exact_repositories(
    client: &Client,
    in_: &Vec<String>,
    out: &Vec<String>,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut repos = Vec::new();
    let mut repos_stream = client.list_repositories().into_paginator().send();
    while let Some(output) = repos_stream.next().await {
        for repo in output.unwrap().repositories.unwrap() {
            let repo_name = repo.repository_name.clone().expect("No repo name");
            if !out.iter().any(|x| repo_name.contains(x))
                && in_.iter().any(|x| repo_name.eq(x))
            {
                repos.push(repo_name);
            }
        }
    }
    debug!("Repositories: {:?}", repos);
    Ok(repos)
}
