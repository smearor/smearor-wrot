use crate::widget::imp::widget::CompositorWidgetImpl;
use dashmap::DashSet;
use gtk4::gdk::Display;
use smithay::backend::allocator::Fourcc;
use tracing::debug;
use tracing::trace;

pub trait DmabufFormats {
    /// Set the supported GTK DMA-BUF formats whitelist
    ///
    /// This should be called during initialization after querying both renderer and GTK formats.
    fn set_supported_gtk_formats(&self, formats: DashSet<(Fourcc, u64)>);

    /// Returns the compatible DMA-BUF formats between the renderer and GTK.
    ///
    /// This function compares the formats supported by the renderer with the formats supported by GTK
    /// and returns only the formats that are supported by both.
    fn get_compatible_dmabuf_formats(&self, _display: &gtk4::gdk::Display, renderer_formats: DashSet<(Fourcc, u64)>) -> DashSet<(Fourcc, u64)>;

    /// Checks if a specific DMA-BUF format is supported by GTK.
    fn is_format_supported_by_gtk(&self, fourcc: Fourcc, modifier: u64) -> bool;
}

impl DmabufFormats for CompositorWidgetImpl {
    fn set_supported_gtk_formats(&self, formats: DashSet<(Fourcc, u64)>) {
        // Clear existing formats and insert new ones
        self.supported_gtk_formats.clear();
        for format in formats {
            self.supported_gtk_formats.insert(format);
        }
        debug!("Set supported GTK DMA-BUF formats");
    }

    fn get_compatible_dmabuf_formats(&self, _display: &Display, renderer_formats: DashSet<(Fourcc, u64)>) -> DashSet<(Fourcc, u64)> {
        // The GTK4 Rust bindings do not provide a direct dmabuf_formats() method.
        // We use a whitelist of the most common formats supported by GTK4/GDK.

        debug!("Querying GTK4 DMA-BUF formats ({} renderer formats provided)", renderer_formats.len());

        // Whitelist of the most common DMA-BUF formats supported by GTK4/GDK
        // These are based on the GTK4 C-API (gdk_display_get_dmabuf_formats) and common formats
        // Modifier: 0 = DRM_FORMAT_MOD_LINEAR (linear tiling)
        let gtk_supported_formats: DashSet<(Fourcc, u64)> = [
            // ARGB8888 (DRM_FORMAT_ARGB8888 = 0x34325258 = 'AR24')
            (Fourcc::Argb8888, 0),
            // XRGB8888 (DRM_FORMAT_XRGB8888 = 0x34325258 = 'XR24')
            (Fourcc::Xrgb8888, 0),
            // ABGR8888 (DRM_FORMAT_ABGR8888 = 0x34324241 = 'AB24')
            (Fourcc::Abgr8888, 0),
            // XBGR8888 (DRM_FORMAT_XBGR8888 = 0x34324258 = 'XB24')
            (Fourcc::Xbgr8888, 0),
            // RGB565 (DRM_FORMAT_RGB565 = 0x36314752)
            (Fourcc::Rgb565, 0),
            // NV12 (DRM_FORMAT_NV12 = 0x3231564e)
            (Fourcc::Nv12, 0),
            // 10-bit 'Deep Color' (HDR-ready formats)
            // ARGB2101010: [A:2, R:10, G:10, B:10] - Little Endian
            // Provides 1024 color steps per channel, essential for reducing
            // banding artifacts in smooth gradients.
            (Fourcc::Argb2101010, 0),
            // BGRA1010102: [B:10, G:10, R:10, A:2] - Little Endian
            // Hardware-optimized layout for specific GPU vendors (e.g., AMD/Intel).
            // Required for full compatibility with modern WebKitGTK accelerated compositing.
            (Fourcc::Bgra1010102, 0),
        ]
        .iter()
        .cloned()
        .collect();

        debug!("GTK4 supports {} DMA-BUF formats in whitelist", gtk_supported_formats.len());

        // Calculate the intersection of formats
        let compatible_formats = DashSet::new();
        for format in renderer_formats {
            if gtk_supported_formats.contains(&format) {
                debug!("Format {:?} (Mod: {:?}) is compatible with GTK4", format.0, format.1);
                compatible_formats.insert(format);
            } else {
                trace!("Format {:?} (Mod: {:?}) is not supported by GTK4 - skipping", format.0, format.1);
            }
        }

        debug!("Found {} compatible DMA-BUF formats between renderer and GTK4", compatible_formats.len());

        compatible_formats
    }

    fn is_format_supported_by_gtk(&self, fourcc: Fourcc, modifier: u64) -> bool {
        // Extract u32 value from Fourcc for DRM code comparison
        // DrmFourcc(AR24) and gdk::Fourcc::Argb8888 are technically identical
        // but incompatible in Rust's type system. Both use standardized DRM values.
        let fourcc_u32 = fourcc as u32;

        // Linear modifier (0) is widely supported and compatible with most hardware
        // Allow all common formats with Linear modifier
        if modifier == 0 {
            match fourcc_u32 {
                // ARGB8888 (DRM_FORMAT_ARGB8888 = 0x34325241 = 'AR24')
                0x34325241 |
                // XRGB8888 (DRM_FORMAT_XRGB8888 = 0x34325258 = 'XR24')
                0x34325258 |
                // ABGR8888 (DRM_FORMAT_ABGR8888 = 0x34324241 = 'AB24')
                0x34324241 |
                // XBGR8888 (DRM_FORMAT_XBGR8888 = 0x34324258 = 'XB24')
                0x34324258 => {
                    return true;
                }
                _ => {}
            }
        }
        self.supported_gtk_formats.contains(&(fourcc, modifier))
    }
}
