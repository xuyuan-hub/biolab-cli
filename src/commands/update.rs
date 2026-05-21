use clap::{Args, Subcommand};
use colored::Colorize;
use serde::{Deserialize, Serialize};

use crate::output::{print_result, OutputFormat};

const LATEST_RELEASE_API: &str =
    "https://api.github.com/repos/xuyuan-hub/biolab-cli/releases/latest";
const USER_AGENT: &str = concat!("biolab-cli/", env!("CARGO_PKG_VERSION"));

#[derive(Args)]
pub struct UpdateArgs {
    #[command(subcommand)]
    pub command: UpdateCommand,
}

#[derive(Subcommand)]
pub enum UpdateCommand {
    /// Check GitHub Releases for a newer CLI version
    Check,
}

#[derive(Debug, Deserialize)]
struct GithubRelease {
    tag_name: String,
    html_url: String,
    assets: Vec<GithubAsset>,
}

#[derive(Debug, Deserialize)]
struct GithubAsset {
    name: String,
    browser_download_url: String,
}

#[derive(Debug, Serialize)]
struct UpdateReport {
    current_version: String,
    latest_version: String,
    update_available: bool,
    release_url: String,
    recommended_assets: Vec<ReleaseAsset>,
}

#[derive(Debug, Serialize)]
struct ReleaseAsset {
    name: String,
    url: String,
}

pub async fn run(args: &UpdateArgs, format: &OutputFormat) -> anyhow::Result<()> {
    match args.command {
        UpdateCommand::Check => {
            let report = check_latest_release().await?;
            print_update_report(&report, format);
        }
    }
    Ok(())
}

async fn check_latest_release() -> anyhow::Result<UpdateReport> {
    let release = reqwest::Client::new()
        .get(LATEST_RELEASE_API)
        .header(reqwest::header::USER_AGENT, USER_AGENT)
        .send()
        .await?
        .error_for_status()?
        .json::<GithubRelease>()
        .await?;

    let current_version = env!("CARGO_PKG_VERSION").to_string();
    let latest_version = release.tag_name.trim_start_matches('v').to_string();
    let update_available = version_gt(&latest_version, &current_version);
    let wanted_assets = recommended_asset_names();
    let recommended_assets = release
        .assets
        .into_iter()
        .filter(|asset| wanted_assets.contains(&asset.name.as_str()))
        .map(|asset| ReleaseAsset {
            name: asset.name,
            url: asset.browser_download_url,
        })
        .collect();

    Ok(UpdateReport {
        current_version,
        latest_version,
        update_available,
        release_url: release.html_url,
        recommended_assets,
    })
}

fn print_update_report(report: &UpdateReport, format: &OutputFormat) {
    match format {
        OutputFormat::Json => print_result(report, format),
        OutputFormat::Text => {
            println!("当前版本: {}", report.current_version);
            println!("最新版本: {}", report.latest_version);

            if report.update_available {
                println!("{}", "发现新版本。".yellow().bold());
                println!("Release: {}", report.release_url);
                for asset in &report.recommended_assets {
                    println!("下载: {}  {}", asset.name, asset.url);
                }
            } else {
                println!("{}", "已是最新版本。".green());
            }
        }
    }
}

fn recommended_asset_names() -> &'static [&'static str] {
    #[cfg(target_os = "windows")]
    {
        &["biolab_win.zip", "biolab_win.zip.sha256"]
    }
    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    {
        &["biolab_mac_arm64", "biolab_mac_arm64.sha256"]
    }
    #[cfg(all(target_os = "macos", not(target_arch = "aarch64")))]
    {
        &["biolab_mac_amd64", "biolab_mac_amd64.sha256"]
    }
    #[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
    {
        &["biolab_unix", "biolab_unix.sha256"]
    }
}

fn version_gt(left: &str, right: &str) -> bool {
    parse_version(left) > parse_version(right)
}

fn parse_version(version: &str) -> Vec<u64> {
    version
        .trim_start_matches('v')
        .split('.')
        .map(|part| {
            part.chars()
                .take_while(|ch| ch.is_ascii_digit())
                .collect::<String>()
                .parse::<u64>()
                .unwrap_or(0)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compares_semver_like_versions() {
        assert!(version_gt("0.2.4", "0.2.3"));
        assert!(version_gt("v0.3.0", "0.2.99"));
        assert!(!version_gt("0.2.3", "0.2.3"));
        assert!(!version_gt("0.2.3", "0.2.4"));
    }
}
