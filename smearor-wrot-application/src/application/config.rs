use smearor_wrot_rotation::layer::SmearorLayer;
use std::ffi::OsString;
use std::path::PathBuf;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, TypedBuilder)]
pub struct CompositorApplicationConfig {
    #[builder(default)]
    pub disable_rotation: bool,

    #[builder(default = 0.0)]
    pub rotation: f32,

    #[builder(default = 1024)]
    pub width: i32,

    #[builder(default = 768)]
    pub height: i32,

    #[builder(default = true)]
    pub decorated: bool,

    #[builder(default = true)]
    pub resizable: bool,

    #[builder(default)]
    pub position_x: Option<i32>,

    #[builder(default)]
    pub position_y: Option<i32>,

    #[builder(default)]
    pub min_width: Option<i32>,

    #[builder(default)]
    pub min_height: Option<i32>,

    #[builder(default)]
    pub max_width: Option<i32>,

    #[builder(default)]
    pub max_height: Option<i32>,

    #[builder(default)]
    pub aspect_ratio: Option<f32>,

    #[builder(default)]
    pub fullscreen: bool,

    #[builder(default)]
    pub maximized: bool,

    #[builder(default = true)]
    pub double_buffer: bool,

    #[builder(default)]
    pub disable_dma_buf: bool,

    #[builder(default)]
    pub id: Option<String>,

    #[builder(default)]
    pub title: Option<String>,

    #[builder(default)]
    pub layer: Option<SmearorLayer>,

    #[builder(default)]
    pub namespace: Option<String>,

    #[builder(default)]
    pub shell: bool,

    #[builder(default)]
    pub socket: Option<String>,

    #[builder(default)]
    pub config_path: Option<PathBuf>,

    #[builder(default)]
    pub wayland_debug: bool,

    #[builder(default)]
    pub gsk_renderer_gl: bool,

    #[builder(default)]
    pub disable_client_decorations: bool,

    #[builder(default)]
    pub margin: Option<u32>,

    #[builder(default = 0)]
    pub margin_left: u32,

    #[builder(default = 0)]
    pub margin_right: u32,

    #[builder(default = 0)]
    pub margin_top: u32,

    #[builder(default = 0)]
    pub margin_bottom: u32,

    #[builder(default = 1.0)]
    pub opacity: f32,

    #[builder(default)]
    pub background_color: Option<String>,

    #[builder(default)]
    pub subsurface_background_color: Option<String>,

    #[builder(default)]
    pub color_mask: Option<String>,

    #[builder(default)]
    pub auto_color_mask: bool,

    #[builder(default)]
    pub subsurface_color_mask: Option<String>,

    #[builder(default)]
    pub auto_subsurface_color_mask: bool,

    #[builder(default = 0.1)]
    pub color_mask_tolerance: f32,

    #[builder(default)]
    pub color_mask_shader: bool,

    #[builder(default = 1.0)]
    pub window_opacity: f32,

    #[builder(default = 60)]
    pub max_fps: i64,

    #[builder(default = 0)]
    pub dialog_margin: u32,

    #[builder(default = 500)]
    pub animation_speed: u64,

    #[builder(default = 1.7)]
    pub animation_overshoot: f64,

    #[builder(default)]
    pub disable_animations: bool,

    #[builder(default)]
    pub debug_touch: bool,

    #[builder(default)]
    pub debug_pointer: bool,

    #[builder(default)]
    pub override_wayland_display: Option<String>,

    #[builder(default)]
    pub keyboard_layout: Option<String>,

    #[builder(default)]
    pub keyboard_variant: Option<String>,

    #[builder(default)]
    pub command_arguments: Vec<OsString>,
}
