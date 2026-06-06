use crate::widget::imp::widget::CompositorWidgetImpl;
use glib::subclass::prelude::ObjectSubclassExt;
use gtk4::prelude::WidgetExt;
use smearor_wrot_core::damage::surface::SurfaceDamage;
use tracing::debug;

impl CompositorWidgetImpl {
    pub fn request_render(&self) {
        self.request_render_internal(false);
    }

    pub fn request_render_force(&self) {
        debug!("request_render_force: forcing render (bypassing damage check)");
        self.request_render_internal(true);
    }

    fn request_render_internal(&self, force: bool) {
        let compositor = self.compositor.borrow();
        let Some(compositor) = compositor.as_ref() else {
            return;
        };

        let Ok(compositor) = compositor.lock() else {
            return;
        };

        // Check for damage regions before queueing draw
        let all_damage = compositor.get_all_surface_damage();

        if !force && all_damage.is_empty() {
            debug!("request_render: no damage regions, skipping queue_draw (GTK4 will use cached render nodes)");
            return;
        }

        // Calculate total damage area using u64 to prevent overflow
        let mut total_damage_area: u64 = 0;
        for region in &all_damage {
            let width = region.size.w as u64;
            let height = region.size.h as u64;
            let area = width.saturating_mul(height);
            total_damage_area = total_damage_area.saturating_add(area);
        }

        debug!(
            "request_render: force={}, found {} damage regions, total area: {} pixels",
            force,
            all_damage.len(),
            total_damage_area
        );

        // Log individual damage regions for debugging
        for (i, region) in all_damage.iter().enumerate() {
            let width = region.size.w as u64;
            let height = region.size.h as u64;
            let area = width.saturating_mul(height);
            debug!(
                "  Region {}: position=({}, {}), size={}x{}, area={} pixels",
                i, region.loc.x, region.loc.y, region.size.w, region.size.h, area
            );
        }

        // Force redraw by invalidating the widget when force rendering
        if force {
            // Force redraw without re-layout to avoid flickering
            self.obj().queue_draw();
        } else {
            self.obj().queue_draw();
        }
    }
}
