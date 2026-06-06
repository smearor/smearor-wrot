//! smearor-wrot-compositor-widget: GTK4 widget for compositor rendering

pub mod background;
pub mod clipboard;
pub mod color;
pub mod color_mask;
pub mod config;
pub mod event_handler;
pub mod opengl_renderer;
pub mod paintable;
pub mod texture;
pub mod wayland_source;
pub mod widget;

pub use color_mask::color_mask_applier::ColorMaskApplier;
pub use color_mask::color_mask_applier::dma_buf::DmaBufColorMaskApplier;
pub use color_mask::color_mask_applier::open_gl::OpenGLColorMaskApplier;
pub use color_mask::color_mask_applier::shm::ShmColorMaskApplier;
pub use config::ColorMask;
pub use config::CompositorWidgetConfig;
pub use paintable::TexturePaintable;
pub use texture::extract_pixel_data_from_texture;
pub use widget::compositor::error::CompositorError;
pub use widget::widget::CompositorWidget;
