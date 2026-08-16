//! Keyboard layout detection

#[cfg(feature = "regex")]
use regex::Regex;
#[cfg(feature = "regex")]
use std::fs::read_to_string;
#[cfg(feature = "regex")]
use std::process::Command;

/// Represents a keyboard layout with its optional variant
#[derive(Debug, Clone)]
pub struct KeyboardLayout {
    /// The keyboard layout identifier (e.g., "de", "us")
    pub layout: String,
    /// The keyboard variant (e.g., "nodeadkeys")
    pub variant: Option<String>,
}

impl KeyboardLayout {
    /// Creates a new keyboard layout
    pub fn new(layout: String, variant: Option<String>) -> Self {
        Self { layout, variant }
    }

    /// Get the full layout name (e.g., "de" or "de(nodeadkeys)")
    pub fn full_name(&self) -> String {
        match &self.variant {
            Some(variant) => format!("{}({})", self.layout, variant),
            None => self.layout.clone(),
        }
    }

    /// Detect the current keyboard layout from the system
    #[cfg(feature = "regex")]
    pub fn detect() -> Option<Self> {
        if let Some(layout) = Self::detect_via_localectl() {
            return Some(layout);
        }
        if let Some(layout) = Self::detect_via_gsettings() {
            return Some(layout);
        }
        if let Some(layout) = Self::detect_via_etc_default_keyboard() {
            return Some(layout);
        }
        None
    }

    #[cfg(feature = "regex")]
    fn detect_via_localectl() -> Option<KeyboardLayout> {
        let output = Command::new("localectl").arg("status").output().ok()?;
        let stdout = String::from_utf8(output.stdout).ok()?;

        for line in stdout.lines() {
            if line.contains("X11 Layout:") {
                let layout = line.split(':').nth(1)?.trim().to_string();
                return Some(KeyboardLayout::new(layout, None));
            }
        }
        None
    }

    #[cfg(feature = "regex")]
    fn detect_via_gsettings() -> Option<KeyboardLayout> {
        let output = Command::new("gsettings")
            .args(["get", "org.gnome.desktop.input-sources", "sources"])
            .output()
            .ok()?;
        let stdout = String::from_utf8(output.stdout).ok()?;

        let re = Regex::new(r"\('xkb',\s*'([^']+)'\)").ok()?;
        if let Some(captures) = re.captures(&stdout) {
            let layout = captures.get(1)?.as_str().to_string();

            let variant_re = Regex::new(r"([^(]+)\(([^)]+)\)").ok()?;
            if let Some(variant_captures) = variant_re.captures(&layout) {
                let layout_name = variant_captures.get(1)?.as_str().to_string();
                let variant = variant_captures.get(2)?.as_str().to_string();
                return Some(KeyboardLayout::new(layout_name, Some(variant)));
            }
            return Some(KeyboardLayout::new(layout, None));
        }
        None
    }

    #[cfg(feature = "regex")]
    fn detect_via_etc_default_keyboard() -> Option<KeyboardLayout> {
        let content = read_to_string("/etc/default/keyboard").ok()?;

        let mut layout = None;
        let mut variant = None;

        for line in content.lines() {
            if line.starts_with("XKBLAYOUT=") {
                let value = line.split('=').nth(1)?;
                layout = Some(value.trim_matches('"').to_string());
            }
            if line.starts_with("XKBVARIANT=") {
                let value = line.split('=').nth(1)?;
                variant = Some(value.trim_matches('"').to_string());
            }
        }

        layout.map(|l| KeyboardLayout::new(l, variant))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyboard_layout_new() {
        let layout = KeyboardLayout::new("de".to_string(), None);
        assert_eq!(layout.layout, "de");
        assert!(layout.variant.is_none());
    }

    #[test]
    fn test_keyboard_layout_full_name_without_variant() {
        let layout = KeyboardLayout::new("us".to_string(), None);
        assert_eq!(layout.full_name(), "us");
    }

    #[test]
    fn test_keyboard_layout_full_name_with_variant() {
        let layout = KeyboardLayout::new("de".to_string(), Some("nodeadkeys".to_string()));
        assert_eq!(layout.full_name(), "de(nodeadkeys)");
    }
}
