//! Keyboard layout detection for GTK4 applications

use regex::Regex;
use std::fs::read_to_string;
use std::process::Command;

/// Represents a keyboard layout with its variant
#[derive(Debug, Clone)]
pub struct KeyboardLayout {
    /// The keyboard layout.
    pub layout: String,

    /// The keyboard variant.
    pub variant: Option<String>,
}

impl KeyboardLayout {
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

    /// Detect the current keyboard layout from GDK
    pub fn detect() -> Option<Self> {
        // Try localectl first
        if let Some(layout) = Self::detect_via_localectl() {
            return Some(layout);
        }

        // Try GSettings for GNOME
        if let Some(layout) = Self::detect_via_gsettings() {
            return Some(layout);
        }

        // Try /etc/default/keyboard
        if let Some(layout) = Self::detect_via_etc_default_keyboard() {
            return Some(layout);
        }

        None
    }

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

    fn detect_via_gsettings() -> Option<KeyboardLayout> {
        let output = Command::new("gsettings")
            .args(["get", "org.gnome.desktop.input-sources", "sources"])
            .output()
            .ok()?;

        let stdout = String::from_utf8(output.stdout).ok()?;

        // Parse output like: [('xkb', 'de'), ('xkb', 'us')]
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
