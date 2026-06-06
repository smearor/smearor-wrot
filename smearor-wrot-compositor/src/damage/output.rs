use smithay::output::Output;
use smithay::utils::Logical;
use smithay::utils::Rectangle;

use crate::compositor::SmearorCompositor;

/// Trait for tracking damage on compositor outputs
///
/// Damage tracking allows the compositor to identify which regions of an output
/// need to be repainted, enabling efficient incremental rendering.
pub trait OutputDamage {
    /// Mark a region of the output as damaged
    ///
    /// # Arguments
    ///
    /// * `output` - The output to mark as damaged
    /// * `region` - The region to mark as damaged, or None to mark the entire output
    fn mark_output_damage(&mut self, output: &Output, region: Option<Rectangle<i32, Logical>>);

    /// Get the damage regions for an output
    ///
    /// # Arguments
    ///
    /// * `output` - The output to get damage regions for
    ///
    /// # Returns
    ///
    /// A vector of damage regions for the specified output
    fn get_output_damage(&self, output: &Output) -> Vec<Rectangle<i32, Logical>>;

    /// Clear all damage regions for an output
    ///
    /// # Arguments
    ///
    /// * `output` - The output to clear damage for
    fn clear_output_damage(&mut self, output: &Output);
}

impl OutputDamage for SmearorCompositor {
    fn mark_output_damage(&mut self, output: &Output, region: Option<Rectangle<i32, Logical>>) {
        let output_name = output.name();
        if let Some(damage_rect) = region {
            self.output_damage.entry(output_name.clone()).or_default().push(damage_rect);
        } else {
            // Mark entire output as damaged by storing a large rectangle
            let entire_damage = Rectangle::new(smithay::utils::Point::new(0, 0), smithay::utils::Size::new(i32::MAX, i32::MAX));
            self.output_damage.insert(output_name.clone(), vec![entire_damage]);
        }
    }

    fn get_output_damage(&self, output: &Output) -> Vec<Rectangle<i32, Logical>> {
        let output_name = output.name();
        self.output_damage.get(&output_name).map(|v| v.value().clone()).unwrap_or_default()
    }

    fn clear_output_damage(&mut self, output: &Output) {
        let output_name = output.name();
        self.output_damage.remove(&output_name);
    }
}
