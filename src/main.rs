use clap::Parser;
use log::info;
use std::fs::File;
use std::io::{self, Write};
mod codecommit;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[clap(
    version = "v0.0.1",
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

    #[clap(short, long, default_value = "anton.sidorov@advarra.com")]
    email: String,

    #[clap(short, long, default_value = "Anton Sidorov")]
    name: String,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    env_logger::init();

    let args = Args::parse();

    let mut file = File::create(&args.file)?;

    let mut include = args.include.clone();
    include.extend(args.base.clone());

    for p in args.profiles.iter() {
        for r in args.regions.iter() {
            let client = codecommit::initialize_client(r, p).await;
            let repositories =
                codecommit::list_repositories(&client, &include, &args.exclude).await;
            info!("Repositories: {:?}", repositories);
            for u in repositories.unwrap() {
                writeln!(
                    file,
                    "[credential \"https://git-codecommit.{}.amazonaws.com/v1/repos/{}.git\"]",
                    r, u
                )?;
                writeln!(
                    file,
                    "\thelper = !aws codecommit credential-helper $@ --profile {}",
                    p
                )?;
                writeln!(file, "\tuseHttpPath = true")?;
            }
        }
    }

    writeln!(
        file,
        "[credential]\n\thelper = !aws codecommit credential-helper $@\n\tUseHttpPath = true"
    )?;
    writeln!(
        file,
        "[user]\n\temail = {}\n\tname = {}",
        &args.email, &args.name
    )?;

    Ok(())
}
