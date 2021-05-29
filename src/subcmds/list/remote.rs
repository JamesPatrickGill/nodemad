use clap::{App, Arg, ArgMatches};
use console::style;

use crate::{
    display::Table,
    fetchers::fetch_node_versions::{
        fetch_local_node_versions, fetch_remote_node_versions, filter_by_lts, filter_by_versions,
    },
    utils::current_version,
};

pub fn create_remote_subcmd() -> App<'static> {
    App::new("remote")
        .about(
            "Get versions available from nodejs.org, displays relevant versions by default (>v10).",
        )
        .arg(
            Arg::new("filter")
                .short('f')
                .long("filter")
                .value_name("CONSTRAINT")
                .about("Sets a constraint on versions displayed. (e.g. >14, =12, <10)")
                .default_value(">12")
                .takes_value(true),
        )
        .arg(
            Arg::new("lts")
                .short('l')
                .long("lts")
                .about("Constrains versions displayed to lts only."),
        )
        .arg(
            Arg::new("all")
                .short('a')
                .long("all")
                .about("Displays all node version available, overriding `filter` flag."),
        )
}

pub async fn run_remote(args: &ArgMatches) -> Result<(), reqwest::Error> {
    let mut node_versions = fetch_remote_node_versions().await?;

    if args.is_present("filter") && !args.is_present("all") {
        filter_by_versions(&mut node_versions, args.value_of("filter").unwrap());
    }

    if args.is_present("lts") {
        filter_by_lts(&mut node_versions);
    }

    let current_version = current_version();
    let installed_versions = fetch_local_node_versions();

    let mut table = Table::new(vec![
        style("Version").bold().to_string(),
        style("Status").bold().to_string(),
    ]);

    node_versions.into_iter().for_each(|version| {
        if version.contains(&current_version) {
            table.add_row(vec![
                style(version).green().bold().to_string(),
                style("active").green().bold().to_string(),
            ]);
        } else if installed_versions.iter().any(|vers| version.contains(vers)) {
            table.add_row(vec![
                style(version).blue().to_string(),
                style("installed").blue().to_string(),
            ]);
        } else {
            table.add_row(vec![style(version).to_string(), "".to_string()]);
        }
    });

    table.print_table();

    Ok(())
}
