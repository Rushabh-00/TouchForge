use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub name: String,
    pub controls: Vec<Control>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Control {
    pub control_type: String,
    pub x: f32,
    pub y: f32,
    pub scale: f32,
    pub binding: String,
}

#[derive(Debug, Error)]
pub enum ProfileError {
    #[error("failed to read profile file: {0}")]
    Io(#[from] std::io::Error),

    #[error("failed to parse profile JSON: {0}")]
    Json(#[from] serde_json::Error),
}

pub fn load_profile<P: AsRef<Path>>(path: P) -> Result<Profile, ProfileError> {
    let text = fs::read_to_string(path)?;
    let profile: Profile = serde_json::from_str(&text)?;
    Ok(profile)
}

pub fn load_profile_text<P: AsRef<Path>>(path: P) -> Result<String, ProfileError> {
    let text = fs::read_to_string(path)?;
    Ok(text)
}
