use std::str::FromStr;
use gtk4_layer_shell::Layer;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SmearorLayer {
    #[serde(alias = "background", alias = "Background", alias = "BACKGROUND")]
    Background,
    #[serde(alias = "bottom", alias = "Bottom", alias = "BOTTOM")]
    Bottom,
    #[serde(alias = "top", alias = "Top", alias = "TOP")]
    Top,
    #[serde(alias = "overlay", alias = "Overlay", alias = "OVERLAY")]
    Overlay,
}

impl Default for SmearorLayer {
    fn default() -> Self {
        SmearorLayer::Top
    }
}

impl From<SmearorLayer> for Layer {
    fn from(layer: SmearorLayer) -> Self {
        match layer {
            SmearorLayer::Background => Layer::Background,
            SmearorLayer::Bottom => Layer::Bottom,
            SmearorLayer::Top => Layer::Top,
            SmearorLayer::Overlay => Layer::Overlay,
        }
    }
}

impl FromStr for SmearorLayer {
    type Err = miette::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "background" => Ok(SmearorLayer::Background),
            "bottom" => Ok(SmearorLayer::Bottom),
            "top" => Ok(SmearorLayer::Top),
            "overlay" => Ok(SmearorLayer::Overlay),
            _ => Ok(SmearorLayer::Top),
        }
    }
}

impl From<&str> for SmearorLayer {

    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "background" => SmearorLayer::Background,
            "bottom" => SmearorLayer::Bottom,
            "top" => SmearorLayer::Top,
            "overlay" => SmearorLayer::Overlay,
            _ => SmearorLayer::Top,
        }
    }
}
