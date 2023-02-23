use anyhow::{Context, Result};
use lair_keystore_api::{
    dependencies::{serde_yaml, url::Url},
    prelude::LairServerConfigInner,
};
use serde::Deserialize;
use std::env;
use std::{fs::File, path::PathBuf};

pub const APP_PORT: u16 = 42233;
// pub const ADMIN_PORT: u16 = 4444;

pub fn default_password() -> Result<String> {
    env::var("HOLOCHAIN_DEFAULT_PASSWORD")
        .context("Failed to read HOLOCHAIN_DEFAULT_PASSWORD. Is it set in env?")
}

pub fn default_core_happ_file() -> Result<String> {
    env::var("CORE_HAPP_FILE").context("Failed to read CORE_HAPP_FILE. Is it set in env?")
}

pub fn get_lair_url() -> Result<Url> {
    let config = read_lair_config()?;
    Ok(config.connection_url)
}

fn read_lair_config() -> Result<LairServerConfigInner> {
    let file = File::open(default_lair_dir()?)?;
    let config: LairServerConfigInner = serde_yaml::from_reader(file)?;
    Ok(config)
}

fn default_lair_dir() -> Result<String> {
    let working_dir = env::var("HOLOCHAIN_WORKING_DIR")
        .context("Failed to read HOLOCHAIN_WORKING_DIR. Is it set in env?")?;
    Ok(format!(
        "{}/lair-keystore/lair-keystore-config.yaml",
        working_dir
    ))
}

/// Configuration of a single hApp from config.yaml
/// ui_path and dna_path takes precedence over ui_url and dna_url respectively
/// and is meant for running tests
#[derive(Debug, Deserialize, Clone)]
pub struct Happ {
    pub bundle_url: Option<Url>,
    pub bundle_path: Option<PathBuf>,
    pub ui_url: Option<Url>,
    pub ui_path: Option<PathBuf>,
}

impl Happ {
    /// generates the installed app id that should be used
    /// based on the path or url of the bundle.
    /// Assumes file name ends in .happ, and converts periods -> colons
    pub fn id(&self) -> String {
        let name = if let Some(ref bundle) = self.bundle_path {
            bundle
                .file_name()
                .unwrap()
                .to_os_string()
                .to_string_lossy()
                .to_string()
        } else if let Some(ref bundle) = self.bundle_url {
            bundle.path_segments().unwrap().last().unwrap().to_string()
        } else {
            "unreachable".to_string()
        };
        if let Ok(uid) = env::var("DEV_UID_OVERRIDE") {
            format!("{}::{}", name.replace(".happ", "").replace('.', ":"), uid)
        } else {
            name.replace(".happ", "").replace('.', ":")
        }
    }
}

/// config with list of core happ for the holoport
#[derive(Debug, Deserialize)]
pub struct HappsFile {
    pub self_hosted_happs: Vec<Happ>,
    pub core_happs: Vec<Happ>,
}

impl HappsFile {
    pub fn holofuel(self) -> Option<Happ> {
        let core_app = &self
            .core_happs
            .into_iter()
            .find(|x| x.id().contains("holofuel"));
        core_app.clone()
    }

    pub fn build() -> Result<Self> {
        let file = File::open(default_core_happ_file()?).context("failed to open file")?;
        let happ_file =
            serde_yaml::from_reader(&file).context("failed to deserialize YAML as HappsFile")?;
        Ok(happ_file)
    }
}
