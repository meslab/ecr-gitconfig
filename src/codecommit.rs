use aws_config::default_provider::credentials::DefaultCredentialsChain;
use aws_sdk_codecommit::config::Region;
use aws_sdk_codecommit::{Client, Config};
use log::debug;

pub async fn initialize_client(region: &str, profile: &str) -> Client {
    let codecommit_region = Region::new(region.to_owned());

    let credentials_provider = DefaultCredentialsChain::builder()
        .region(codecommit_region.clone())
        .profile_name(profile)
        .build()
        .await;
    let config = Config::builder()
        .credentials_provider(credentials_provider)
        .region(codecommit_region)
        .build();

    Client::from_conf(config)
}

pub async fn list_filtered_repositories<F>(
    client: &Client,
    filter: F,
) -> Result<Vec<String>, Box<dyn std::error::Error>>
where
    F: Fn(&String) -> bool + Send + Sync,
{
    let mut repos = Vec::new();
    let mut repos_stream = client.list_repositories().into_paginator().send();
    while let Some(output) = repos_stream.next().await {
        for repo in output?.repositories.unwrap() {
            let repo_name = repo.repository_name.unwrap();
            if filter(&repo_name) {
                repos.push(repo_name);
            }
        }
    }
    debug!("Repositories: {:?}", repos);
    Ok(repos)
}

pub async fn list_repositories(
    client: &Client,
    in_: &[String],
    out: &[String],
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let in_: Vec<_> = in_.iter().map(|x| x.as_str()).collect();
    let out: Vec<_> = out.iter().map(|x| x.as_str()).collect();
    let filter = |repo_name: &String| {
        !out.iter().any(|x| repo_name.contains(x)) && in_.iter().any(|x| repo_name.contains(x))
    };
    list_filtered_repositories(client, filter).await
}

pub async fn list_exact_repositories(
    client: &Client,
    in_: &[String],
    out: &[String],
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let in_: Vec<_> = in_.iter().map(|x| x.as_str()).collect();
    let out: Vec<_> = out.iter().map(|x| x.as_str()).collect();
    let filter = |repo_name: &String| {
        !out.iter().any(|x| repo_name.contains(x)) && in_.iter().any(|x| repo_name.eq(x))
    };
    list_filtered_repositories(client, filter).await
}
