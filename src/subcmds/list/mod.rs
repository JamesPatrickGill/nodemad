mod local;
mod remote;

use local::{create_local_subcmd, run_local};
use remote::{create_remote_subcmd, run_remote};

use clap::App;
use clap::ArgMatches;

pub fn create_list_subcmd() -> App<'static> {
    let remote_subcmd = create_remote_subcmd();
    let local_subcmd = create_local_subcmd();

    App::new("list")
        .about("controls listing features")
        .subcommand(remote_subcmd)
        .subcommand(local_subcmd)
}

pub async fn run_list(args: &ArgMatches) -> Result<(), reqwest::Error> {
    match args.subcommand() {
        Some(("remote", sub_args)) => {
            run_remote(sub_args).await?;
        }
        Some(("local", sub_args)) => {
            run_local(sub_args);
        }
        _ => {
            println!("list called with {:?}", args);
        }
    };
    Ok(())
}
