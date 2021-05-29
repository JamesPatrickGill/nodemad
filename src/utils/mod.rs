use home::home_dir;
use reqwest::Url;

pub fn download_url(version: &str) -> Result<Url, Box<dyn std::error::Error>> {
    let os_specific_file = format!("node-{}-darwin-x64.tar.gz", version);
    let download_url_str = format!("https://nodejs.org/dist/{}/{}", version, os_specific_file);
    Ok(Url::parse(download_url_str.as_str())?)
}

pub fn node_archive_name(version: &str) -> String {
    format!("node-{}-darwin-x64.tar.gz", version)
}

pub fn node_file_name(version: &str) -> String {
    format!("node-{}-darwin-x64", version)
}

pub fn install_dir() -> String {
    format!(
        "{}/.nodemad/installed",
        home_dir().unwrap().to_str().unwrap(),
    )
}

pub fn format_version_arg(arg_val: &str) -> String {
    if arg_val.starts_with("v") {
        arg_val.to_string()
    } else {
        format!("v{}", arg_val)
    }
}

pub fn current_version() -> String {
    let link = format!("{}/.nodemad/current", home_dir().unwrap().to_str().unwrap());

    let dir = std::fs::read_link(link).unwrap();
    let file = dir.file_name().unwrap().to_os_string();
    file.into_string().unwrap()
}
