use anyhow::{anyhow, Result};
use octocrab;
use reqwest::{header, redirect};
use serde::Deserialize;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::os::unix::fs::symlink;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
struct ReleaseAsset {
    url: String,
    name: String,
    browser_download_url: String,
}

pub fn get_local_versions(soli_dir: &PathBuf) -> Result<Vec<String>> {
    fs::create_dir_all(soli_dir)?;
    Ok(fs::read_dir(soli_dir)?
        .filter(|path_res| path_res.as_ref().is_ok())
        .filter(|path_res| {
            path_res
                .as_ref()
                .map_or(false, |r| r.metadata().map_or(false, |m| m.is_dir()))
        })
        .filter_map(|path_res| path_res.ok().map(|path| path.file_name()))
        .filter_map(|os_str| os_str.to_str().map(|s| s.to_string()))
        .collect())
}

pub fn get_current_version(exe_dir: &PathBuf) -> Result<String> {
    let mut exe_file = exe_dir.clone();
    exe_file.push("solc");
    let actual_file = fs::canonicalize(exe_file)?;
    let directory = actual_file.parent().expect("Invalid file location");
    let directory_path = directory
        .file_name()
        .ok_or(anyhow!("Invalid directory path"))?;
    let version = directory_path
        .to_str()
        .map(|s| s.to_string())
        .ok_or(anyhow!("Invalid file location"))?;
    Ok(version)
}

pub async fn get_remote_versions() -> Result<Vec<String>> {
    let octocrab = octocrab::instance();
    Ok(octocrab
        .repos("ethereum", "solidity")
        .releases()
        .list()
        .send()
        .await?
        .items
        .iter()
        .filter_map(|release| {
            release
                .clone()
                .name
                .filter(|name| name.contains("Version"))
                .map(|name| name.replace("Version ", ""))
        })
        .collect())
}

pub fn uninstall_version(soli_dir: &PathBuf, version: &str) -> Result<()> {
    let mut version_dir = soli_dir.clone();
    version_dir.push(version);
    fs::remove_dir_all(version_dir)?;
    Ok(())
}

pub fn use_version(soli_dir: &PathBuf, exe_dir: &PathBuf, version: &str) -> Result<()> {
    let local_versions = get_local_versions(soli_dir)?;
    if !local_versions.contains(&version.to_string()) {
        return Err(anyhow!("Version {} not installed", version));
    }

    let solc_file = get_solc_file(soli_dir, version)?;

    let mut exe_file = exe_dir.clone();
    exe_file.push("solc");

    fs::remove_file(&exe_file).ok();
    symlink(solc_file, &exe_file)?;

    Ok(())
}

pub async fn install_version(soli_dir: &PathBuf, version: &str) -> Result<()> {
    let local_versions = get_local_versions(soli_dir)?;
    if local_versions.contains(&version.to_string()) {
        return Err(anyhow!("Version {} already installed", version));
    }

    install_from_github(soli_dir, version).await?;

    Ok(())
}

async fn get_github_asset_url(version: &str) -> Result<String> {
    let octocrab = octocrab::instance();
    let release: Option<String> = octocrab
        .repos("ethereum", "solidity")
        .releases()
        .list()
        .send()
        .await?
        .items
        .iter()
        .filter(|release| {
            release
                .name
                .clone()
                .filter(|name| name.contains("Version"))
                .map(|name| name.replace("Version ", ""))
                .filter(|name| name == version)
                .is_some()
        })
        .map(|release| release.assets_url.clone().to_string())
        .next();

    if release.is_none() {
        return Err(anyhow!(format!("No remote version {}", version)));
    }

    return Ok(release.unwrap());
}

fn get_solc_file(soli_dir: &PathBuf, version: &str) -> Result<PathBuf> {
    let mut download_dir = soli_dir.clone();
    download_dir.push(version);
    fs::create_dir_all(&download_dir)?;
    let mut download_file = download_dir.clone();
    download_file.push("solc");
    Ok(download_file)
}

fn get_download_file(soli_dir: &PathBuf, version: &str) -> Result<(PathBuf, File)> {
    let download_file = get_solc_file(soli_dir, version)?;
    Ok((download_file.clone(), File::create(download_file)?))
}

async fn install_from_github(soli_dir: &PathBuf, version: &str) -> Result<()> {
    let asset_url = get_github_asset_url(version).await?;

    let mut headers = header::HeaderMap::new();
    headers.insert("User-Agent", header::HeaderValue::from_static("Soli"));
    let client = reqwest::Client::builder()
        .redirect(redirect::Policy::limited(3))
        .default_headers(headers)
        .build()?;

    let download_url: Option<String> = client
        .get(asset_url)
        .send()
        .await?
        .json::<Vec<ReleaseAsset>>()
        .await?
        .iter()
        .filter(|asset| asset.name == "solc-static-linux")
        .map(|asset| asset.browser_download_url.clone())
        .next();

    if download_url.is_none() {
        return Err(anyhow!(format!(
            "No asset download for version {}",
            version
        )));
    }

    let download = client
        .get(download_url.unwrap())
        .send()
        .await?
        .bytes()
        .await?;

    let (download_file, mut dest) = get_download_file(soli_dir, version)?;

    dest.write(&download)?;
    fs::set_permissions(&download_file, fs::Permissions::from_mode(0o770))?;

    Ok(())
}
