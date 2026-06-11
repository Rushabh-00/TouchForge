pub mod converter;
pub mod model;
pub mod touchforge;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct IcpProfile {
    pub id: i32,
    pub name: String,

    #[serde(rename = "cursorSpeed")]
    pub cursor_speed: f32,

    pub elements: Vec<IcpElement>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IcpElement {
    #[serde(rename = "type")]
    pub element_type: String,

    pub shape: String,

    pub bindings: Vec<String>,

    pub scale: f32,

    pub x: f32,
    pub y: f32,

    #[serde(rename = "toggleSwitch")]
    pub toggle_switch: bool,

    pub text: String,

    #[serde(rename = "iconId")]
    pub icon_id: i32,

    #[serde(default)]
    pub range: Option<String>,
}

pub fn load_icp<P: AsRef<Path>>(path: P) -> Result<IcpProfile> {
    let text = fs::read_to_string(path)?;
    let profile: IcpProfile = serde_json::from_str(&text)?;
    Ok(profile)
}

pub fn save_icp<P: AsRef<Path>>(
    profile: &IcpProfile,
    path: P,
) -> Result<()> {
    let json = serde_json::to_string_pretty(profile)?;
    fs::write(path, json)?;
    Ok(())
}

pub fn export_json<P: AsRef<Path>>(
    profile: &TfProfile,
    path: P,
) -> Result<()> {
    let json = serde_json::to_string_pretty(profile)?;
    fs::write(path, json)?;
    Ok(())
}

pub use converter::*;
pub use model::*;
pub use touchforge::*;
