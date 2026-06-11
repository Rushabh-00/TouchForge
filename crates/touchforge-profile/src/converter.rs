use crate::{
    IcpProfile,
    InternalElement,
    InternalElementType,
    InternalProfile,
    TfProfile,
};

pub fn tfp_to_internal(profile: &TfProfile) -> InternalProfile {
    InternalProfile {
        name: profile.name.clone(),
        elements: profile
            .elements
            .iter()
            .map(|e| InternalElement {
                id: e.id.clone(),

                element_type: match e.element_type {
                    crate::ElementType::Button => InternalElementType::Button,
                    crate::ElementType::Joystick => InternalElementType::Joystick,
                    crate::ElementType::SwipeArea => InternalElementType::SwipeArea,
                    crate::ElementType::MouseArea => InternalElementType::MouseArea,

                    crate::ElementType::RangeButton => InternalElementType::Button,
                    crate::ElementType::DPad => InternalElementType::Button,
                    crate::ElementType::Trigger => InternalElementType::Button,
                },

                x: e.x,
                y: e.y,

                width: e.width,
                height: e.height,
            })
            .collect(),
    }
}

pub fn icp_to_internal(profile: &IcpProfile) -> InternalProfile {
    InternalProfile {
        name: profile.name.clone(),

        elements: profile
            .elements
            .iter()
            .enumerate()
            .map(|(index, element)| {
                let element_type =
                    match element.element_type.to_lowercase().as_str() {
                        "joystick" => InternalElementType::Joystick,
                        "swipearea" => InternalElementType::SwipeArea,
                        "mousearea" => InternalElementType::MouseArea,
                        _ => InternalElementType::Button,
                    };

                InternalElement {
                    id: format!("element_{}", index),

                    element_type,

                    x: element.x,
                    y: element.y,

                    width: element.scale * 100.0,
                    height: element.scale * 100.0,
                }
            })
            .collect(),
    }
}
