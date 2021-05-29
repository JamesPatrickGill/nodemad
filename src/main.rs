mod display;
mod fetchers;
mod subcmds;
mod utils;

use subcmds::{
    install::{create_install_subcmd, run_install},
    list::{create_list_subcmd, run_list},
    r#use::{create_use_subcmd, run_use},
};

use clap::App;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let list_subcmd = create_list_subcmd();
    let install_subcmd = create_install_subcmd();
    let use_subcmd = create_use_subcmd();

    let matches = App::new("nodemad")
        .version("1.0")
        .about("A quick and simple way to jump between node versions.")
        .subcommand(list_subcmd)
        .subcommand(install_subcmd)
        .subcommand(use_subcmd)
        .get_matches();

    match matches.subcommand() {
        Some(("list", sub_args)) => {
            run_list(sub_args).await?;
        }
        Some(("install", sub_args)) => {
            run_install(sub_args).await?;
        }
        Some(("use", sub_args)) => {
            run_use(sub_args).await?;
        }
        _ => {
            println!("nodemad called with")
        }
    };
    Ok(())
}
