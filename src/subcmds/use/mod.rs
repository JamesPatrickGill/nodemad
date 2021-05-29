use std::{fs::remove_file, os::unix::fs::symlink, path::Path};

use clap::{App, Arg, ArgMatches};

use home::home_dir;

pub fn create_use_subcmd() -> App<'static> {
    App::new("use")
        .about("Set version of node to be used.")
        .arg(
            Arg::new("node_version")
                .about("Node version to use.")
                .required(true),
        )
}

pub async fn run_use(args: &ArgMatches) -> Result<(), reqwest::Error> {
    let node_version_arg = String::from(args.value_of("node_version").unwrap());
    let version = if node_version_arg.starts_with("v") {
        node_version_arg
    } else {
        format!("v{}", node_version_arg)
    };
    let version_str = version.as_str();

    let original = format!(
        "{}/.nodemad/installed/{}",
        home_dir().unwrap().to_str().unwrap(),
        version_str
    );

    let link = format!("{}/.nodemad/current", home_dir().unwrap().to_str().unwrap());

    let orig_path = Path::new(original.as_str());
    let orig_link = Path::new(link.as_str());

    if orig_link.exists() || !orig_link.is_dir() {
        remove_file(orig_link).unwrap();
    }

    symlink(orig_path, orig_link).unwrap();

    Ok(())
}
