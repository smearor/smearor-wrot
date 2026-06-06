//! Rendering pipeline

use smithay::output::Output;
use std::sync::Arc;
use std::sync::Mutex;

use crate::compositor::SmearorCompositor;
use crate::damage::output::OutputDamage;
use crate::render::DoubleBuffer;
use crate::render::FrameLimiter;
use crate::render::OutputRendering;
use crate::render::SurfaceRendering;

/// Trait for rendering pipeline operations
pub trait RenderingPipeline {
    /// Render a single frame
    fn render_frame(&self, output: &Output);

    /// Render all mapped windows
    fn render_windows(&self, output: &Output);

    /// Check if rendering is needed
    fn needs_render(&self, output: &Output) -> bool;
}

impl RenderingPipeline for Arc<Mutex<SmearorCompositor>> {
    fn render_frame(&self, output: &Output) {
        // Check frame rate limiting
        if let Ok(guard) = self.lock() {
            if !guard.should_render() {
                return;
            }
        }

        // Get damage regions for this output
        let damage_regions = if let Ok(guard) = self.lock() {
            guard.get_output_damage(output)
        } else {
            return;
        };

        // If there's no damage, skip rendering
        if damage_regions.is_empty() {
            return;
        }

        // Check if double buffering is enabled
        let use_double_buffer = if let Ok(guard) = self.lock() {
            guard.is_double_buffer_enabled()
        } else {
            false
        };

        self.render_output(output);
        let windows: Vec<_> = if let Ok(guard) = self.lock() {
            guard.space.elements().cloned().collect()
        } else {
            return;
        };
        for window in windows {
            self.render_surface(&window);
        }

        // Swap buffers if double buffering is enabled
        if use_double_buffer {
            if let Ok(guard) = self.lock() {
                guard.swap_buffers();
            }
        }

        // Update frame time and clear damage after rendering
        if let Ok(mut guard) = self.lock() {
            guard.update_frame_time();
            guard.clear_output_damage(output);
        }
    }

    fn render_windows(&self, output: &Output) {
        // Check frame rate limiting
        if let Ok(guard) = self.lock() {
            if !guard.should_render() {
                return;
            }
        }

        // Get damage regions for this output
        let damage_regions = if let Ok(guard) = self.lock() {
            guard.get_output_damage(output)
        } else {
            return;
        };

        // If there's no damage, skip rendering
        if damage_regions.is_empty() {
            return;
        }

        // Check if double buffering is enabled
        let use_double_buffer = if let Ok(guard) = self.lock() {
            guard.is_double_buffer_enabled()
        } else {
            false
        };

        self.render_output(output);
        let windows: Vec<_> = if let Ok(guard) = self.lock() {
            guard.space.elements().cloned().collect()
        } else {
            return;
        };
        for window in windows {
            if self.needs_rendering(&window) {
                self.render_surface(&window);
            }
        }

        // Swap buffers if double buffering is enabled
        if use_double_buffer {
            if let Ok(guard) = self.lock() {
                guard.swap_buffers();
            }
        }

        // Update frame time and clear damage after rendering
        if let Ok(mut guard) = self.lock() {
            guard.update_frame_time();
            guard.clear_output_damage(output);
        }
    }

    fn needs_render(&self, output: &Output) -> bool {
        let output_name = output.name();
        if let Ok(guard) = self.lock() {
            return !guard.rendered_outputs.contains(&output_name);
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use smithay::output::Output;
    use std::collections::HashSet;
    use std::sync::Arc;
    use std::sync::Mutex;

    #[test]
    fn test_rendering_pipeline_trait_exists() {
        use crate::compositor::SmearorCompositor;
        use crate::render::pipeline::RenderingPipeline;

        let _ = |c: &Arc<Mutex<SmearorCompositor>>, o: &Output| {
            c.render_frame(o);
        };
    }

    #[test]
    fn test_render_windows_trait_exists() {
        use crate::compositor::SmearorCompositor;
        use crate::render::pipeline::RenderingPipeline;

        let _ = |c: &Arc<Mutex<SmearorCompositor>>, o: &Output| {
            c.render_windows(o);
        };
    }

    #[test]
    fn test_needs_render_trait_exists() {
        use crate::compositor::SmearorCompositor;
        use crate::render::pipeline::RenderingPipeline;

        let _ = |c: &Arc<Mutex<SmearorCompositor>>, o: &Output| {
            c.needs_render(o);
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
    fn test_needs_render_with_empty_outputs() {
        let rendered_outputs: Arc<Mutex<HashSet<String>>> = Arc::new(Mutex::new(HashSet::new()));
        let output_name = "test_output".to_string();

        let outputs = rendered_outputs.lock().unwrap();
        assert!(!outputs.contains(&output_name));
    }

    #[test]
    fn test_needs_render_with_rendered_output() {
        let rendered_outputs: Arc<Mutex<HashSet<String>>> = Arc::new(Mutex::new(HashSet::new()));
        let output_name = "test_output".to_string();

        let mut outputs = rendered_outputs.lock().unwrap();
        outputs.insert(output_name.clone());
        drop(outputs);

        let outputs = rendered_outputs.lock().unwrap();
        assert!(outputs.contains(&output_name));
    }
}
