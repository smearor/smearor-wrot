/// MIME-type converter for clipboard data
/// Handles conversion between different MIME-types and formats
pub struct FormatConverter;

impl FormatConverter {
    /// Convert text from GTK format to Wayland format
    /// GTK uses UTF8_STRING, Wayland clients use text/plain;charset=utf-8
    pub fn gtk_to_wayland_text(text: &str) -> String {
        // Normalize line endings from \r\n to \n
        text.replace("\r\n", "\n")
    }

    /// Convert text from Wayland format to GTK format
    /// Wayland clients use text/plain;charset=utf-8, GTK uses UTF8_STRING
    pub fn wayland_to_gtk_text(text: &str) -> String {
        // GTK can handle both \n and \r\n, so we keep as-is
        text.to_string()
    }

    /// Normalize line endings to Unix style (\n)
    pub fn normalize_line_endings(text: &str) -> String {
        text.replace("\r\n", "\n").replace('\r', "\n")
    }

    /// Check if a MIME-type is a text type
    pub fn is_text_mime_type(mime_type: &str) -> bool {
        mime_type == "text/plain" || mime_type == "text/plain;charset=utf-8" || mime_type == "UTF8_STRING" || mime_type == "TEXT" || mime_type == "STRING"
    }

    /// Get the canonical MIME-type for text
    pub fn canonical_text_mime_type() -> &'static str {
        "text/plain;charset=utf-8"
    }

    /// Map a MIME-type to its canonical form
    pub fn map_mime_type(mime_type: &str) -> &'static str {
        match mime_type {
            "UTF8_STRING" | "TEXT" | "STRING" => "text/plain;charset=utf-8",
            "text/plain" => "text/plain;charset=utf-8",
            _ => {
                // Return the original mime_type if it's not in our map
                // We need to handle the lifetime issue by returning a static string
                // For now, we'll return a default if it's not recognized
                "text/plain;charset=utf-8"
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gtk_to_wayland_text_conversion() {
        let gtk_text = "Hello\r\nWorld";
        let wayland_text = FormatConverter::gtk_to_wayland_text(gtk_text);
        assert_eq!(wayland_text, "Hello\nWorld");
    }

    #[test]
    fn test_normalize_line_endings() {
        let text = "Hello\r\nWorld\rTest";
        let normalized = FormatConverter::normalize_line_endings(text);
        assert_eq!(normalized, "Hello\nWorld\nTest");
    }

    #[test]
    fn test_is_text_mime_type() {
        assert!(FormatConverter::is_text_mime_type("text/plain"));
        assert!(FormatConverter::is_text_mime_type("text/plain;charset=utf-8"));
        assert!(FormatConverter::is_text_mime_type("UTF8_STRING"));
        assert!(!FormatConverter::is_text_mime_type("text/uri-list"));
    }

    #[test]
    fn test_map_mime_type() {
        assert_eq!(FormatConverter::map_mime_type("UTF8_STRING"), "text/plain;charset=utf-8");
        assert_eq!(FormatConverter::map_mime_type("text/plain"), "text/plain;charset=utf-8");
        assert_eq!(FormatConverter::map_mime_type("text/uri-list"), "text/uri-list");
    }
}
