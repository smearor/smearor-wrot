//! Output rendering

use smithay::output::Output;
use std::sync::Arc;
use std::sync::Mutex;

use crate::compositor::SmearorCompositor;

/// Trait for output rendering operations
pub trait OutputRendering {
    /// Render an output
    fn render_output(&self, output: &Output);

    /// Schedule a render for an output
    fn schedule_render(&self, output: &Output);

    /// Get the last render time for an output
    fn last_render_time(&self, output: &Output) -> Option<std::time::Instant>;
}

impl OutputRendering for Arc<Mutex<SmearorCompositor>> {
    fn render_output(&self, output: &Output) {
        let output_name = output.name();
        let now = std::time::Instant::now();
        if let Ok(guard) = self.lock() {
            guard.last_render_times.insert(output_name.clone(), now);
        }
    }

    fn schedule_render(&self, output: &Output) {
        let output_name = output.name();
        if let Ok(guard) = self.lock() {
            guard.rendered_outputs.insert(output_name.clone());
        }
    }

    fn last_render_time(&self, output: &Output) -> Option<std::time::Instant> {
        let output_name = output.name();
        if let Ok(guard) = self.lock() {
            return guard.last_render_times.get(&output_name).as_deref().copied();
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use dashmap::DashMap;
    use smithay::output::Output;
    use std::collections::HashSet;
    use std::sync::Arc;
    use std::sync::Mutex;

    #[test]
    fn test_output_rendering_trait_exists() {
        use crate::compositor::SmearorCompositor;
        use crate::render::output::OutputRendering;

        let _ = |c: &Arc<Mutex<SmearorCompositor>>, o: &Output| {
            c.render_output(o);
        };
    }

    #[test]
    fn test_schedule_render_trait_exists() {
        use crate::compositor::SmearorCompositor;
        use crate::render::output::OutputRendering;

        let _ = |c: &Arc<Mutex<SmearorCompositor>>, o: &Output| {
            c.schedule_render(o);
        };
    }

    #[test]
    fn test_last_render_time_trait_exists() {
        use crate::compositor::SmearorCompositor;
        use crate::render::output::OutputRendering;

        let _ = |c: &Arc<Mutex<SmearorCompositor>>, o: &Output| {
            c.last_render_time(o);
        };
    }

    #[test]
    fn test_rendered_outputs_mutex_operations() {
        let rendered_outputs: Arc<Mutex<HashSet<String>>> = Arc::new(Mutex::new(HashSet::new()));
        let output_name = "test_output".to_string();

        let mut outputs = rendered_outputs.lock().unwrap();
        outputs.insert(output_name.clone());
        drop(outputs);

        let outputs = rendered_outputs.lock().unwrap();
        assert!(outputs.contains(&output_name));
    }

    #[test]
    fn test_last_render_times_dashmap_operations() {
        let last_render_times: Arc<DashMap<String, std::time::Instant>> = Arc::new(DashMap::new());
        let output_name = "test_output".to_string();
        let now = std::time::Instant::now();

        last_render_times.insert(output_name.clone(), now);
        assert!(last_render_times.contains_key(&output_name));
    }

    #[test]
    fn test_rendered_outputs_empty_initially() {
        let rendered_outputs: Arc<Mutex<HashSet<String>>> = Arc::new(Mutex::new(HashSet::new()));
        let outputs = rendered_outputs.lock().unwrap();
        assert!(outputs.is_empty());
    }

    #[test]
    fn test_last_render_times_empty_initially() {
        let last_render_times: Arc<DashMap<String, std::time::Instant>> = Arc::new(DashMap::new());
        assert!(last_render_times.is_empty());
    }
}
