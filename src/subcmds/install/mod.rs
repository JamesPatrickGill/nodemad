use std::{
    fs::{remove_file, rename, File},
    path::Path,
};

use clap::{App, Arg, ArgMatches};

use indicatif::ProgressBar;
use tar::Archive;

use crate::{
    display::{confirm, get_spinner_style},
    fetchers::{
        fetch_node_archive::fetch_node_archive, fetch_node_versions::fetch_local_node_versions,
    },
    subcmds::r#use::run_use,
    utils::{format_version_arg, install_dir, node_archive_name, node_file_name},
};

use console::style;

pub fn create_install_subcmd() -> App<'static> {
    App::new("install")
        .about("Install a version of nodejs to your machine.")
        .arg(
            Arg::new("node_version")
                .about("Node version to install.")
                .required(true),
        )
}

pub async fn run_install(args: &ArgMatches) -> Result<(), reqwest::Error> {
    let version = format_version_arg(args.value_of("node_version").unwrap());

    let local_versions = fetch_local_node_versions();

    if local_versions.iter().any(|vers| vers == &version) {
        let prompt_message = format!(
            "Node version {} is already installed. Would you like to make this your active version?",
            style(&version).green().bold()
        );

        if confirm(prompt_message) {
            run_use(args).await?;
            return Ok(());
        } else {
            return Ok(());
        }
    };

    fetch_node_archive(&version).await.unwrap();
    extract_file(&version).unwrap();

    Ok(())
}

fn extract_file(version: &str) -> Result<(), Box<dyn std::error::Error>> {
    let progress_bar = ProgressBar::new_spinner();
    progress_bar.enable_steady_tick(100);
    progress_bar.set_style(get_spinner_style());
    progress_bar.set_prefix("Extracting");

    let compressed_archive =
        File::open(format!("{}/{}", install_dir(), node_archive_name(version))).unwrap();

    let tar = flate2::read::GzDecoder::new(compressed_archive);
    let mut archive = Archive::new(tar);
    archive.unpack(install_dir()).unwrap();

    rename(
        format!("{}/{}", install_dir(), node_file_name(version)),
        format!("{}/{}", install_dir(), version),
    )
    .unwrap();

    remove_file(Path::new(
        format!("{}/{}", install_dir(), node_archive_name(version)).as_str(),
    ))
    .unwrap();

    progress_bar.set_message("Success");
    progress_bar.finish_at_current_pos();

    Ok(())
}
