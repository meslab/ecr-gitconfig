use clap::Parser;
use log::info;
use std::fs::File;
use std::io::{self, Write};
mod codecommit;
use git2::Config;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[clap(
    version = "v0.1.0",
    author = "Anton Sidorov tonysidrock@gmail.com",
    about = "Counts wwords frequency in a text file"
)]
struct Args {
    #[clap(short, long, default_value = "/tmp/gitconfig")]
    file: String,

    #[clap(short, long, required = true)]
    base: Vec<String>,

    #[clap(short, long, default_value = None)]
    include: Vec<String>,

    #[clap(short = 'x', long, default_value = None)]
    exclude: Vec<String>,

    #[clap(short, long, default_values = &["infra"])]
    profiles: Vec<String>,

    #[clap(short, long, default_values = &["eu-central-1", "us-east-2"])]
    regions: Vec<String>,

    #[clap(short, long, default_value = None)]
    email: Option<String>,

    #[clap(short, long, default_value = None)]
    name: Option<String>,
}

fn write_gitconfig(
    file: &mut File,
    repo: &str,
    region: &str,
    profile: &str,
) -> Result<(), std::io::Error> {
    writeln!(
        file,
        "[credential \"https://git-codecommit.{}.amazonaws.com/v1/repos/{}.git\"]",
        region, repo
    )?;
    writeln!(
        file,
        "\thelper = !aws codecommit credential-helper $@ --profile {}",
        profile
    )?;
    writeln!(file, "\tuseHttpPath = true")
}

#[tokio::main]
async fn main() -> io::Result<()> {
    env_logger::init();

    let args = Args::parse();

    let cfg = Config::open_default().unwrap();
    let binding = cfg.get_string("user.name").unwrap();
    let username = match &args.name {
        Some(name) => name,
        None => &binding,
    };
    let binding = cfg.get_string("user.email").unwrap();
    let email = match &args.email {
        Some(email) => email,
        None => &binding,
    };

    let mut file = File::create(&args.file)?;

    for profile in args.profiles.iter() {
        for region in args.regions.iter() {
            let client = codecommit::initialize_client(&region, &profile).await;
            let base_repositories =
                codecommit::list_exact_repositories(&client, &args.base, &args.exclude).await;
            info!("Base repositories: {:?}", base_repositories);
            let repositories =
                codecommit::list_repositories(&client, &args.include, &args.exclude).await;
            info!("Repositories: {:?}", repositories);
            for repo in base_repositories.unwrap() {
                write_gitconfig(&mut file, &repo, region, profile).expect("Cannt write to file")
            }
            for repo in repositories.unwrap() {
                write_gitconfig(&mut file, &repo, region, profile).expect("Cannt write to file")
            }
        }
    }

    writeln!(
        file,
        "[credential]\n\thelper = !aws codecommit credential-helper $@\n\tUseHttpPath = true"
    )?;
    writeln!(file, "[user]\n\temail = {}\n\tname = {}", &email, &username)?;

    Ok(())
}
