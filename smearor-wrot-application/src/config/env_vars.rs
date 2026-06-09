use std::sync::atomic::AtomicBool;
use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder)]
pub struct EnvironmentVariablesState {
    #[builder(default)]
    pub env_override_wayland_debug: AtomicBool,

    #[builder(default)]
    pub env_override_wayland_display: Option<String>,
}
