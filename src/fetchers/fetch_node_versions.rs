use reqwest::Response;
use serde::Deserialize;
use serde_json::Value;
use std::fs;

use crate::utils::install_dir;

#[derive(Deserialize, Debug)]
pub struct NodeVersion {
    pub version: String,
    pub lts: Value,
}

pub async fn fetch_remote_node_versions() -> Result<Vec<String>, reqwest::Error> {
    let client = reqwest::Client::new();
    let resp: Response = client
        .get("https://nodejs.org/dist/index.json")
        .send()
        .await?;
    let mut body = resp.json::<Vec<NodeVersion>>().await?;
    body.reverse();

    let version_vec: Vec<String> = body
        .iter()
        .map(|version| {
            if version.lts.is_string() {
                return format!(
                    "{} - (lts: {})",
                    version.version,
                    version.lts.as_str().unwrap()
                );
            } else {
                return format!("{}", version.version);
            };
        })
        .collect();

    Ok(version_vec)
}

pub fn filter_by_versions(all_versions: &mut Vec<String>, constraint: &str) {
    match constraint.chars().nth(0) {
        Some('>') => {
            let index = all_versions
                .iter()
                .position(|version| *(&(version)[1..].starts_with(&constraint[1..])))
                .unwrap();
            all_versions.drain(..index);
        }
        Some('<') => {
            let index = all_versions
                .iter()
                .position(|version| *(&(version)[1..].starts_with(&constraint[1..])))
                .unwrap();
            all_versions.truncate(index);
        }

        _ => todo!(),
    }
}

pub fn filter_by_lts(all_versions: &mut Vec<String>) {
    all_versions.retain(|version| version.contains("lts"))
}

pub fn fetch_local_node_versions() -> Vec<String> {
    let paths = fs::read_dir(install_dir()).unwrap();

    let mut versions: Vec<String> = paths
        .into_iter()
        .map(|path| {
            path.unwrap()
                .path()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string()
        })
        .collect();

    versions.reverse();

    versions
}
