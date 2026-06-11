use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternalProfile {
    pub name: String,
    pub elements: Vec<InternalElement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternalElement {
    pub id: String,
    pub element_type: InternalElementType,

    pub x: f32,
    pub y: f32,

    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InternalElementType {
    Button,
    Joystick,
    SwipeArea,
    MouseArea,
}
