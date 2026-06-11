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

pub fn load_icp<P: AsRef<Path>>(
    path: P,
) -> Result<IcpProfile, Box<dyn std::error::Error>> {
    let text = fs::read_to_string(path)?;
    let profile: IcpProfile = serde_json::from_str(&text)?;
    Ok(profile)
}
