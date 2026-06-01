//! smearor-wrot-core: Compositor functionality for process rendering

pub mod background;
pub mod buffer;
pub mod callback;
pub mod clipboard;
pub mod color_mask;
pub mod commit;
pub mod compositor;
pub mod damage;
pub mod dialog;
pub mod dma;
pub mod error;
pub mod frame;
pub mod handlers;
pub mod input;
pub mod lifecycle;
pub mod logging;
pub mod margin;
pub mod message;
pub mod output;
pub mod popup;
pub mod render;
pub mod state;
pub mod subsurface;
pub mod surface;
pub mod texture;
pub mod wayland;
pub mod windows;
pub mod winit;

pub use buffer::BufferImportExport;
pub use buffer::BufferLifecycle;
pub use buffer::BufferTracking;
pub use compositor::CalloopData;
pub use compositor::SmearorCompositor;
pub use dma::buffer::DmaBuffer;
pub use error::CoreError;
pub use logging::init_logging;
pub use logging::init_logging_with_level;
pub use output::OutputGeometry;
pub use render::DoubleBuffer;
pub use render::OutputRendering;
pub use render::RenderingPipeline;
pub use render::SurfaceRendering;
pub use state::ClientState;

pub const DEFAULT_WINDOW_WIDTH: i32 = 1200;
pub const DEFAULT_WINDOW_HEIGHT: i32 = 1200;
