use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TfProfile {
    pub name: String,
    pub version: u32,
    pub elements: Vec<TfElement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TfElement {
    pub id: String,

    pub element_type: ElementType,

    pub x: f32,
    pub y: f32,

    pub width: f32,
    pub height: f32,

    pub opacity: f32,

    pub bindings: Vec<String>,

    pub icon_id: Option<i32>,

    pub toggle_switch: bool,

    pub label: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ElementType {
    Button,
    Joystick,
    SwipeArea,
    MouseArea,
    RangeButton,
    DPad,
    Trigger,
}

pub fn save_profile<P: AsRef<Path>>(
    profile: &TfProfile,
    path: P,
) -> Result<()> {
    let json = serde_json::to_string_pretty(profile)?;
    fs::write(path, json)?;
    Ok(())
}

pub fn load_profile<P: AsRef<Path>>(
    path: P,
) -> Result<TfProfile> {
    let text = fs::read_to_string(path)?;
    let profile: TfProfile = serde_json::from_str(&text)?;
    Ok(profile)
}

impl TfProfile {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            version: 2,
            elements: Vec::new(),
        }
    }

    pub fn add_element(&mut self, element: TfElement) {
        self.elements.push(element);
    }
}

impl TfElement {
    pub fn button(
        id: impl Into<String>,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    ) -> Self {
        Self {
            id: id.into(),
            element_type: ElementType::Button,

            x,
            y,

            width,
            height,

            opacity: 1.0,

            bindings: Vec::new(),

            icon_id: None,

            toggle_switch: false,

            label: None,
        }
    }

    pub fn joystick(
        id: impl Into<String>,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    ) -> Self {
        Self {
            id: id.into(),
            element_type: ElementType::Joystick,

            x,
            y,

            width,
            height,

            opacity: 1.0,

            bindings: Vec::new(),

            icon_id: None,

            toggle_switch: false,

            label: None,
        }
    }

    pub fn trigger(
        id: impl Into<String>,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    ) -> Self {
        Self {
            id: id.into(),
            element_type: ElementType::Trigger,

            x,
            y,

            width,
            height,

            opacity: 1.0,

            bindings: Vec::new(),

            icon_id: None,

            toggle_switch: false,

            label: None,
        }
    }
}
