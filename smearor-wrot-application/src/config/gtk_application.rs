use std::sync::atomic::AtomicI64;
use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder)]
pub struct GtkApplicationConfig {
    /// The GTK application id.
    #[builder(default)]
    pub id: Option<String>,

    /// Maximum frames per second (default: 60).
    #[builder(default = 60)]
    pub max_fps: AtomicI64,
}
