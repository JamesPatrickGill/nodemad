use clap::{App, ArgMatches};
use console::style;

use crate::{
    display::Table,
    fetchers::fetch_node_versions::{fetch_local_node_versions, filter_by_lts, filter_by_versions},
    utils::current_version,
};

pub fn create_local_subcmd() -> App<'static> {
    App::new("local").about("Get versions installed on your local machine")
}

pub fn run_local(args: &ArgMatches) {
    let mut versions = fetch_local_node_versions();

    versions.sort();

    if args.is_present("filter") && !args.is_present("all") {
        filter_by_versions(&mut versions, args.value_of("filter").unwrap());
    }

    if args.is_present("lts") {
        filter_by_lts(&mut versions);
    }

    let current_version = current_version();

    let mut table = Table::new(vec![
        style("Version").bold().to_string(),
        style("Status").bold().to_string(),
    ]);

    versions.into_iter().for_each(|version| {
        if current_version == version {
            table.add_row(vec![
                style(version).green().bold().to_string(),
                style("active").green().bold().to_string(),
            ]);
        } else {
            table.add_row(vec![
                style(version).blue().to_string(),
                style("installed").blue().to_string(),
            ]);
        }
    });

    table.print_table();
}
