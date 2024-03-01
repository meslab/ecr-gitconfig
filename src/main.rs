use clap::Parser;
use log::info;
use std::fs::File;
use std::io::{self, Write};
use std::process::Command;
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
}

#[tokio::main]
async fn main() -> io::Result<()> {
    env_logger::init();

    let args = Args::parse();

    let mut file = File::create(&args.file)?;

    for u in ["app", "sandbox", "sandbox-external"].iter() {
        writeln!(
            file,
            "[credential \"https://git-codecommit.eu-central-1.amazonaws.com/v1/repos/{}.git\"]",
            u
        )?;
        writeln!(
            file,
            "  helper = !aws codecommit credential-helper $@ --profile cloud-prod-controlplane"
        )?;
        writeln!(file, "  useHttpPath = true")?;
    }

    let profiles = ["cloud-prod-controlplane", "infra"];
    let regions = ["eu-central-1", "us-east-2"];

    for p in profiles.iter() {
        for r in regions.iter() {
            let output = Command::new("aws")
                .arg("codecommit")
                .arg("list-repositories")
                .arg("--region")
                .arg(r)
                .arg("--profile")
                .arg(p)
                .arg("--query")
                .arg("repositories[?(contains(repositoryName,`-cirbi`) || contains(repositoryName,`-lb`) || contains(repositoryName,`longboat`)) && !contains(repositoryName,`lb1`)].repositoryName")
                .arg("--output")
                .arg("text")
                .output()
                .expect("failed to execute process");

            let repositories = String::from_utf8_lossy(&output.stdout);
            for u in repositories.split_whitespace() {
                writeln!(
                    file,
                    "[credential \"https://git-codecommit.{}.amazonaws.com/v1/repos/{}.git\"]",
                    r, u
                )?;
                writeln!(
                    file,
                    "  helper = !aws codecommit credential-helper $@ --profile {}",
                    p
                )?;
                writeln!(file, "  useHttpPath = true")?;
            }
        }
    }

    writeln!(
        file,
        "[credential]\n  helper = !aws codecommit credential-helper $@\n  UseHttpPath = true"
    )?;
    writeln!(
        file,
        "[user]\n  email = anton.sidorov@advarra.com\n  name = Anton Sidorov"
    )?;

    Ok(())
}
