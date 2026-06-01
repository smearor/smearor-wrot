use crate::rotation::SmearorRotation;
use gtk4::Orientation;
use gtk4::graphene::Point;
use gtk4::prelude::*;
use gtk4::subclass::prelude::*;
use tracing::error;

mod imp {
    use super::*;
    use crate::animation::Animation;
    use crate::animation::EasingFunction;
    use crate::animation::RotationZoomAnimation;
    use glib::ControlFlow;
    use gtk4::PropagationPhase;
    use gtk4::gsk::Transform;
    use std::cell::Cell;
    use std::cell::RefCell;
    use std::cmp::Ordering;
    use std::rc::Rc;
    use std::time::Duration;
    use tracing::debug;
    use tracing::error;
    use tracing::trace;

    pub struct RotatedLayout {
        pub rotation: Cell<f32>,
        pub scale: Cell<f32>,
    }

    impl Default for RotatedLayout {
        fn default() -> Self {
            Self {
                rotation: Cell::new(0.0),
                scale: Cell::new(1.0),
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for RotatedLayout {
        const NAME: &'static str = "RotatedLayout";
        type Type = super::RotatedLayout;
        type ParentType = gtk4::LayoutManager;
    }

    impl ObjectImpl for RotatedLayout {}

    impl LayoutManagerImpl for RotatedLayout {
        fn allocate(&self, widget: &gtk4::Widget, width: i32, height: i32, baseline: i32) {
            let mut child = widget.first_child();
            let angle_deg = self.rotation.get();
            let current_scale = self.scale.get();
            let angle_rad = angle_deg.to_radians();
            let abs_cos = angle_rad.cos().abs();
            let abs_sin = angle_rad.sin().abs();
            while let Some(ref c) = child {
                if c.should_layout() {
                    let denom = abs_cos * abs_cos - abs_sin * abs_sin;
                    let (target_child_w, target_child_h) = if denom.abs() < 0.9 {
                        // Sonderfall 45°, 135° etc. (Matrix singulär)
                        // Hier vereinfacht: Kind quadratisch einpassen
                        let s = (width as f32 / (abs_cos + abs_sin)) as i32;
                        (s, s)
                    } else {
                        let w_c = (width as f32 * abs_cos - height as f32 * abs_sin) / denom;
                        let h_c = (height as f32 * abs_cos - width as f32 * abs_sin) / denom;
                        (w_c as i32, h_c as i32)
                    };

                    // let (_, child_nat_w, _, _) = c.measure(Orientation::Horizontal, -1);
                    // let (_, child_nat_h, _, _) = c.measure(Orientation::Vertical, -1);
                    // trace!("child_nat_w {child_nat_w} child_nat_h {child_nat_h}");
                    let transform = Transform::new()
                        .translate(&Point::new(width as f32 / 2.0, height as f32 / 2.0))
                        .rotate(angle_deg)
                        .scale(current_scale, current_scale)
                        // .translate(&Point::new(child_nat_w as f32 / -2.0, child_nat_h as f32 / -2.0));
                        .translate(&Point::new(target_child_w as f32 / -2.0, target_child_h as f32 / -2.0));

                    trace!("width {target_child_w} height {target_child_h} baseline {baseline} transform {transform:?}");
                    // c.allocate(width, height, baseline, Some(transform));
                    c.allocate(target_child_w, target_child_h, baseline, Some(transform));
                }
                child = c.next_sibling();
            }
        }

        fn measure(&self, widget: &gtk4::Widget, orientation: Orientation, _for_size: i32) -> (i32, i32, i32, i32) {
            let angle_deg = self.rotation.get();
            let angle_rad = angle_deg.to_radians();
            let abs_cos = angle_rad.cos().abs();
            let abs_sin = angle_rad.sin().abs();

            if let Some(child) = widget.first_child() {
                let (child_min_w, child_nat_w, _, _) = child.measure(Orientation::Horizontal, -1);
                let (child_min_h, child_nat_h, _, _) = child.measure(Orientation::Vertical, -1);
                trace!("child_min_w {child_min_w} child_nat_w {child_nat_w} child_min_h {child_min_h} child_nat_h {child_nat_h}");

                if orientation == Orientation::Horizontal {
                    // Calculate bounding box width
                    let min = (child_min_w as f32 * abs_cos + child_min_h as f32 * abs_sin).ceil() as i32;
                    let nat = (child_nat_w as f32 * abs_cos + child_nat_h as f32 * abs_sin).ceil() as i32;
                    (min, nat, -1, -1)
                } else {
                    // Calculate bounding box height
                    let min = (child_min_w as f32 * abs_sin + child_min_h as f32 * abs_cos).ceil() as i32;
                    let nat = (child_nat_w as f32 * abs_sin + child_nat_h as f32 * abs_cos).ceil() as i32;
                    (min, nat, -1, -1)
                }
            } else {
                debug!("No child");
                (0, 0, -1, -1)
            }
        }
    }

    #[derive(Default)]
    pub struct RotationWidget {
        pub child: RefCell<Option<gtk4::Widget>>,
        pub animation: RefCell<Option<Animation>>,
        pub animation_speed: Cell<u64>,
        pub rotation_zoom_animation: RefCell<Option<RotationZoomAnimation>>,
        pub animations_enabled: Cell<bool>,
        pub animation_overshoot: Cell<f64>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for RotationWidget {
        const NAME: &'static str = "RotatedBox";
        type Type = super::RotationWidget;
        type ParentType = gtk4::Widget;

        fn class_init(klass: &mut Self::Class) {
            klass.set_layout_manager_type::<super::RotatedLayout>();
        }
    }

    impl ObjectImpl for RotationWidget {
        fn constructed(&self) {
            self.parent_constructed();

            // Add gesture recognizers for rotation and pinch
            let widget = self.obj();

            // Rotate gesture
            let rotate_gesture = gtk4::GestureRotate::new();
            rotate_gesture.set_propagation_phase(PropagationPhase::Bubble);
            widget.add_controller(rotate_gesture.clone().upcast::<gtk4::EventController>());

            let widget_clone = widget.clone();
            let initial_rotation = Rc::new(RefCell::new(0.0f32));
            let total_angle_delta = Rc::new(RefCell::new(0.0f32));
            let last_angle = Rc::new(RefCell::new(0.0f32));
            let min_angle_threshold = 45.0f32; // Minimum angle threshold in degrees
            let feedback_rotation = Rc::new(RefCell::new(0.0f32));
            let direction_determined = Rc::new(RefCell::new(false));
            let rotation_applied = Rc::new(RefCell::new(false));
            let initial_rotation_clone = initial_rotation.clone();
            let total_angle_delta_clone = total_angle_delta.clone();
            let last_angle_clone = last_angle.clone();
            let feedback_rotation_clone = feedback_rotation.clone();
            let direction_determined_clone = direction_determined.clone();
            let rotation_applied_clone = rotation_applied.clone();

            rotate_gesture.connect_begin(move |_gesture, _sequence| {
                let layout = match widget_clone.layout_manager() {
                    Some(lm) => lm,
                    None => {
                        error!("Failed to get layout manager in rotate gesture begin");
                        return;
                    }
                };

                let layout = match layout.downcast::<super::RotatedLayout>() {
                    Ok(rl) => rl,
                    Err(_) => {
                        error!("Layout manager is not RotatedLayout in rotate gesture begin");
                        return;
                    }
                };

                *initial_rotation_clone.borrow_mut() = layout.imp().rotation.get();
                *total_angle_delta_clone.borrow_mut() = 0.0;
                *last_angle_clone.borrow_mut() = 0.0;
                *feedback_rotation_clone.borrow_mut() = 0.0;
                *direction_determined_clone.borrow_mut() = false;
                *rotation_applied_clone.borrow_mut() = false;
            });

            let widget_clone = widget.clone();
            let initial_rotation_clone = initial_rotation.clone();
            let total_angle_delta_clone = total_angle_delta.clone();
            let last_angle_clone = last_angle.clone();
            let feedback_rotation_clone = feedback_rotation.clone();
            let direction_determined_clone = direction_determined.clone();
            let rotation_applied_clone = rotation_applied.clone();
            // Prevents gesture conflicts with pointer events
            rotate_gesture.set_propagation_phase(PropagationPhase::Capture);
            rotate_gesture.connect_angle_changed(move |_gesture, _angle, angle_delta| {
                debug!("angle_delta {angle_delta}");
                // Stop processing if rotation was already applied
                if *rotation_applied_clone.borrow() {
                    return;
                }

                // 1. Convert to degrees (0.0 to 360.0)
                let current_angle = angle_delta.to_degrees() as f32;

                // 2. Calculate the real delta
                // We need the difference between the current angle and the last angle
                let mut last_angle_ref = last_angle_clone.borrow_mut();

                // Smooth the jump at 0/360 degrees:
                let mut delta = current_angle - *last_angle_ref;

                // If the jump is greater than 180 degrees, we have crossed the 0-degree line
                if delta > 180.0 {
                    delta -= 360.0; // Left rotation across the zero boundary
                } else if delta < -180.0 {
                    delta += 360.0; // Right rotation across the zero boundary
                }

                *last_angle_ref = current_angle;

                // 3. Now 'delta' is reliably positive (CW) or negative (CCW)
                *total_angle_delta_clone.borrow_mut() += delta;

                let current_total = *total_angle_delta_clone.borrow();

                // let mut current_angle = angle_delta.to_degrees();
                //
                // let delta_degrees = angle_delta.to_degrees();
                // *total_angle_delta_clone.borrow_mut() += delta_degrees as f32;
                //
                // let current_total = *total_angle_delta_clone.borrow();

                debug!("current_total {} min_angle_threshold {}", current_total, min_angle_threshold);

                // Determine direction and apply feedback rotation on first significant movement
                if !*direction_determined_clone.borrow() && current_total.abs() > 10.0 {
                    *direction_determined_clone.borrow_mut() = true;
                    let feedback_amount = if current_total > 0.0 { 2.0 } else { -2.0 };
                    *feedback_rotation_clone.borrow_mut() = feedback_amount as f32;

                    debug!("Direction determined: {}, feedback_amount: {}", current_total, feedback_amount);

                    let layout = match widget_clone.layout_manager() {
                        Some(lm) => lm,
                        None => {
                            error!("Failed to get layout manager in rotate gesture feedback");
                            return;
                        }
                    };

                    let layout = match layout.downcast::<super::RotatedLayout>() {
                        Ok(rl) => rl,
                        Err(_) => {
                            error!("Layout manager is not RotatedLayout in rotate gesture feedback");
                            return;
                        }
                    };

                    let current_rotation = layout.imp().rotation.get();
                    let new_rotation = current_rotation + feedback_amount as f32;
                    debug!("Applying feedback: current_rotation {} -> new_rotation {}", current_rotation, new_rotation);
                    layout.imp().rotation.set(new_rotation);
                    widget_clone.queue_allocate();
                    debug!("Applied feedback rotation: {} degrees", feedback_amount);
                }

                if current_total.abs() < min_angle_threshold {
                    // Don't apply rotation until minimum threshold is reached
                    return;
                }

                let layout = match widget_clone.layout_manager() {
                    Some(lm) => lm,
                    None => {
                        error!("Failed to get layout manager in rotate gesture");
                        return;
                    }
                };

                let layout = match layout.downcast::<super::RotatedLayout>() {
                    Ok(rl) => rl,
                    Err(_) => {
                        error!("Layout manager is not RotatedLayout in rotate gesture");
                        return;
                    }
                };

                let current_rotation = layout.imp().rotation.get();
                let initial_rot = *initial_rotation_clone.borrow();

                // Find next standard rotation based on direction
                let normalized_rot = initial_rot.rem_euclid(360.0);
                debug!("current_rotation: {current_rotation} initial_rot: {initial_rot} normalized_rot: {normalized_rot} current_total: {current_total}");
                let new_rotation = if current_total > 0.0 {
                    // Clockwise direction
                    if normalized_rot < 45.0 {
                        90.0f32
                    } else if normalized_rot < 135.0 {
                        180.0
                    } else if normalized_rot < 225.0 {
                        270.0
                    } else {
                        0.0
                    }
                } else {
                    // Counter-clockwise direction
                    if normalized_rot < 45.0 {
                        270.0
                    } else if normalized_rot < 135.0 {
                        0.0
                    } else if normalized_rot < 225.0 {
                        90.0
                    } else {
                        180.0
                    }
                };
                let final_rotation = new_rotation.rem_euclid(360.0);

                debug!(
                    "Animation: initial_rot={} final_rotation={} current_rotation={} normalized_rot={}",
                    initial_rot, final_rotation, current_rotation, normalized_rot
                );

                // Check if animations are enabled
                let animations_enabled = widget_clone.imp().animations_enabled.get();

                if !animations_enabled {
                    // If animations are disabled, set rotation immediately
                    let layout = match widget_clone.layout_manager() {
                        Some(lm) => lm,
                        None => return,
                    };

                    let layout = match layout.downcast::<super::RotatedLayout>() {
                        Ok(rl) => rl,
                        Err(_) => return,
                    };

                    layout.imp().rotation.set(final_rotation as f32);
                    widget_clone.queue_allocate();
                    *rotation_applied_clone.borrow_mut() = true;
                    return;
                }

                // Start overshoot animation using RotationZoomAnimation
                let animation_speed_ms = widget_clone.imp().animation_speed.get();
                let overshoot_amount = widget_clone.imp().animation_overshoot.get();
                let mut animation = RotationZoomAnimation::new(
                    current_rotation as f64,
                    final_rotation as f64,
                    1.0,
                    0.8,
                    1.0,
                    Duration::from_millis(animation_speed_ms),
                    EasingFunction::Overshoot { overshoot_amount },
                );
                animation.start();
                *widget_clone.imp().rotation_zoom_animation.borrow_mut() = Some(animation);

                // Start animation tick callback
                widget_clone.add_tick_callback(move |widget, _frame_clock| {
                    let imp = widget.imp();
                    if let Some(ref mut anim) = *imp.rotation_zoom_animation.borrow_mut() {
                        let (current_rotation, current_scale) = match anim.get_current_values_with_phases() {
                            Some(values) => values,
                            None => return ControlFlow::Break,
                        };

                        let layout = match widget.layout_manager() {
                            Some(lm) => lm,
                            None => return ControlFlow::Continue,
                        };
                        let layout = match layout.downcast::<super::RotatedLayout>() {
                            Ok(rl) => rl,
                            Err(_) => return ControlFlow::Continue,
                        };

                        // Apply rotation
                        layout.imp().rotation.set(current_rotation as f32);

                        // Apply scale using transform matrix instead of CSS
                        layout.imp().scale.set(current_scale as f32);

                        widget.queue_allocate();

                        if anim.is_complete() {
                            // Reset scale to 1.0 after animation completes
                            layout.imp().scale.set(1.0);
                            ControlFlow::Break
                        } else {
                            ControlFlow::Continue
                        }
                    } else {
                        ControlFlow::Break
                    }
                });

                // Mark rotation as applied to prevent further rotations
                *rotation_applied_clone.borrow_mut() = true;
            });

            let widget_clone = widget.clone();
            let initial_rotation_clone = initial_rotation.clone();
            let total_angle_delta_clone = total_angle_delta.clone();
            rotate_gesture.connect_end(move |_gesture, _sequence| {
                let current_total = *total_angle_delta_clone.borrow();

                if current_total.abs() < min_angle_threshold {
                    // Reset to initial rotation if minimum threshold was not reached
                    let layout = match widget_clone.layout_manager() {
                        Some(lm) => lm,
                        None => {
                            error!("Failed to get layout manager in rotate gesture end");
                            return;
                        }
                    };

                    let layout = match layout.downcast::<super::RotatedLayout>() {
                        Ok(rl) => rl,
                        Err(_) => {
                            error!("Layout manager is not RotatedLayout in rotate gesture end");
                            return;
                        }
                    };

                    let initial_rot = *initial_rotation_clone.borrow();
                    layout.imp().rotation.set(initial_rot);
                    widget_clone.queue_allocate();
                    debug!("Reset to initial rotation {initial_rot} because threshold was not reached");
                }
            });
        }

        fn dispose(&self) {
            if let Some(child) = self.child.borrow_mut().take() {
                child.unparent();
            }
        }
    }
    impl WidgetImpl for RotationWidget {
        fn size_allocate(&self, width: i32, height: i32, baseline: i32) {
            self.parent_size_allocate(width, height, baseline);
            let widget = self.obj();
            if let Some(surface) = widget.native().and_then(|n| n.surface()) {
                if let Some(child) = widget.first_child() {
                    let angle = widget
                        .layout_manager()
                        .and_then(|lm| lm.downcast::<super::RotatedLayout>().ok())
                        .map(|rl| rl.imp().rotation.get())
                        .unwrap_or(0.0);
                    let (_, child_w, _, _) = child.measure(Orientation::Horizontal, -1);
                    let (_, child_h, _, _) = child.measure(Orientation::Vertical, -1);
                    let region = self.calculate_rotated_region_with_scale(width as f32, height as f32, child_w as f32, child_h as f32, angle);
                    surface.set_input_region(Some(&region));
                }
            }
        }

        fn snapshot(&self, snapshot: &gtk4::Snapshot) {
            let widget = self.obj();
            if let Some(child) = widget.first_child() {
                widget.snapshot_child(&child, snapshot);
            }
        }
    }

    impl RotationWidget {
        fn calculate_rotated_region_with_scale(&self, win_w: f32, win_h: f32, c_w: f32, c_h: f32, deg: f32) -> gtk4::cairo::Region {
            use std::f32::consts::PI;
            let rad = deg * PI / 180.0;
            let (sin, cos) = rad.sin_cos();
            let _scale_x = win_w / c_w;
            let _scale_y = win_h / c_h;
            let pts = [
                (-win_w / 2.0, -win_h / 2.0),
                (win_w / 2.0, -win_h / 2.0),
                (win_w / 2.0, win_h / 2.0),
                (-win_w / 2.0, win_h / 2.0),
            ];
            let mut transformed = Vec::new();
            for (x, y) in pts {
                let rx = x * cos - y * sin + win_w / 2.0;
                let ry = x * sin + y * cos + win_h / 2.0;
                transformed.push((rx, ry));
            }
            let min_y = transformed.iter().map(|(_, y)| *y).fold(f32::INFINITY, f32::min).floor() as i32;
            let max_y = transformed.iter().map(|(_, y)| *y).fold(f32::NEG_INFINITY, f32::max).ceil() as i32;
            let region = gtk4::cairo::Region::create();
            let _step = 1;
            for y in min_y..max_y {
                let current_y = y as f32 + 0.5; // Middle of the line
                let mut intersections = Vec::new();
                for i in 0..4 {
                    let p1 = transformed[i];
                    let p2 = transformed[(i + 1) % 4];
                    if (p1.1 <= current_y && p2.1 > current_y) || (p2.1 <= current_y && p1.1 > current_y) {
                        let x = p1.0 + (current_y - p1.1) * (p2.0 - p1.0) / (p2.1 - p1.1);
                        intersections.push(x);
                    }
                }

                if intersections.len() >= 2 {
                    intersections.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
                    let start_x = intersections[0];
                    let end_x = intersections[intersections.len() - 1];

                    let rect = gtk4::cairo::RectangleInt::new(start_x.round() as i32, y, (end_x - start_x).round() as i32, 1);
                    if let Err(e) = region.union_rectangle(&rect) {
                        error!("Failed to union rectangle region: {}", e);
                    }
                }
            }

            region
        }
    }
}

glib::wrapper! {
    pub struct RotationWidget(ObjectSubclass<imp::RotationWidget>)
        @extends gtk4::Widget,
        @implements gtk4::Accessible, gtk4::Buildable, gtk4::ConstraintTarget;
}

glib::wrapper! {
    pub struct RotatedLayout(ObjectSubclass<imp::RotatedLayout>)
        @extends gtk4::LayoutManager;
}

impl RotationWidget {
    pub fn new(rotation: SmearorRotation) -> Self {
        let obj: Self = glib::Object::builder().build();
        obj.set_rotation(rotation);
        obj
    }

    pub fn set_rotation(&self, rotation: SmearorRotation) {
        let layout = match self.layout_manager() {
            Some(lm) => lm,
            None => {
                error!("Failed to get layout manager in set_rotation");
                return;
            }
        };

        let layout = match layout.downcast::<RotatedLayout>() {
            Ok(rl) => rl,
            Err(_) => {
                error!("Layout manager is not RotatedLayout in set_rotation");
                return;
            }
        };

        layout.imp().rotation.set(rotation.to_degrees());
        self.queue_allocate();
    }

    pub fn rotation(&self) -> f32 {
        let layout = match self.layout_manager() {
            Some(lm) => lm,
            None => {
                error!("Failed to get layout manager in rotation");
                return 0.0;
            }
        };

        let layout = match layout.downcast::<RotatedLayout>() {
            Ok(rl) => rl,
            Err(_) => {
                error!("Layout manager is not RotatedLayout in rotation");
                return 0.0;
            }
        };

        layout.imp().rotation.get()
    }

    /// Transform input coordinates based on current rotation
    pub fn input_transform(&self, x: f64, y: f64) -> (f64, f64) {
        let rotation = self.rotation();
        if rotation == 0.0 {
            (x, y)
        } else {
            // Get widget dimensions
            let width = self.width() as f64;
            let height = self.height() as f64;

            // Translate to center
            let cx = width / 2.0;
            let cy = height / 2.0;

            let dx = x - cx;
            let dy = y - cy;

            // Rotate in reverse direction to compensate for widget rotation
            let radians = (-rotation).to_radians() as f64;
            let cos = radians.cos();
            let sin = radians.sin();

            let rotated_x = dx * cos - dy * sin;
            let rotated_y = dx * sin + dy * cos;

            // Translate back
            (rotated_x + cx, rotated_y + cy)
        }
    }

    pub fn set_animation_speed(&self, speed_ms: u64) {
        self.imp().animation_speed.set(speed_ms);
    }

    pub fn set_animations_enabled(&self, enabled: bool) {
        self.imp().animations_enabled.set(enabled);
    }

    pub fn set_animation_overshoot(&self, overshoot: f64) {
        self.imp().animation_overshoot.set(overshoot);
    }

    pub fn set_rotation_with_animation(&self, new_rotation: f64) {
        use crate::animation::EasingFunction;
        use crate::animation::RotationZoomAnimation;
        use crate::widget::RotatedLayout;
        use std::time::Duration;

        let imp = self.imp();

        // Check if animations are disabled
        if !imp.animations_enabled.get() {
            // If animations are disabled, set rotation immediately
            let layout = match self.layout_manager() {
                Some(lm) => lm,
                None => {
                    error!("Failed to get layout manager in set_rotation_with_animation");
                    return;
                }
            };

            let layout = match layout.downcast::<RotatedLayout>() {
                Ok(rl) => rl,
                Err(_) => {
                    error!("Layout manager is not RotatedLayout in set_rotation_with_animation");
                    return;
                }
            };

            layout.imp().rotation.set(new_rotation as f32);
            self.queue_allocate();
            return;
        }

        let layout = match self.layout_manager() {
            Some(lm) => lm,
            None => {
                error!("Failed to get layout manager in set_rotation_with_animation");
                return;
            }
        };

        let layout = match layout.downcast::<RotatedLayout>() {
            Ok(rl) => rl,
            Err(_) => {
                error!("Layout manager is not RotatedLayout in set_rotation_with_animation");
                return;
            }
        };

        let current_rotation = layout.imp().rotation.get();
        let animation_speed_ms = imp.animation_speed.get();

        // Create rotation zoom animation with three phases
        // 0-33%: Rotation and zoom out (scale 1.0 -> 0.9)
        // 33-66%: Rotation without zoom (scale 0.9 -> 0.9)
        // 66-100%: Rotation and zoom in (scale 0.9 -> 1.0)
        let mut animation = RotationZoomAnimation::new(
            current_rotation as f64,
            new_rotation,
            1.0,
            0.8,
            1.0,
            Duration::from_millis(animation_speed_ms),
            EasingFunction::EaseInOut,
        );
        animation.start();
        *imp.rotation_zoom_animation.borrow_mut() = Some(animation);

        self.add_tick_callback(move |widget, _frame_clock| {
            let imp = widget.imp();
            if let Some(ref mut anim) = *imp.rotation_zoom_animation.borrow_mut() {
                let (current_rotation, current_scale): (f64, f64) = match anim.get_current_values_with_phases() {
                    Some(values) => values,
                    None => return glib::ControlFlow::Break,
                };

                let layout = match widget.layout_manager() {
                    Some(lm) => lm,
                    None => return glib::ControlFlow::Continue,
                };

                let layout = match layout.downcast::<RotatedLayout>() {
                    Ok(rl) => rl,
                    Err(_) => return glib::ControlFlow::Continue,
                };

                // Apply rotation
                layout.imp().rotation.set(current_rotation as f32);

                // Apply scale using transform matrix instead of CSS
                layout.imp().scale.set(current_scale as f32);

                widget.queue_allocate();

                if anim.is_complete() {
                    // Reset scale to 1.0 after animation completes
                    layout.imp().scale.set(1.0);
                    glib::ControlFlow::Break
                } else {
                    glib::ControlFlow::Continue
                }
            } else {
                glib::ControlFlow::Break
            }
        });
    }

    pub fn set_child(&self, child: Option<&impl IsA<gtk4::Widget>>) {
        let mut self_child = self.imp().child.borrow_mut();
        if let Some(old_child) = self_child.take() {
            old_child.unparent();
        }
        if let Some(new_child) = child {
            let widget_clone_width = self.clone();
            let widget_clone_height = self.clone();
            new_child.connect_notify_local(Some("width-request"), move |_child, _param| {
                widget_clone_width.queue_allocate();
            });
            new_child.connect_notify_local(Some("height-request"), move |_child, _param| {
                widget_clone_height.queue_allocate();
            });
            new_child.set_parent(self);
            *self_child = Some(new_child.clone().upcast());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smearor_rotation_deg0() {
        let rotation = SmearorRotation::Deg0;
        assert_eq!(rotation.to_degrees(), 0.0);
    }

    #[test]
    fn test_smearor_rotation_deg90() {
        let rotation = SmearorRotation::Deg90;
        assert_eq!(rotation.to_degrees(), 90.0);
    }

    #[test]
    fn test_smearor_rotation_deg180() {
        let rotation = SmearorRotation::Deg180;
        assert_eq!(rotation.to_degrees(), 180.0);
    }

    #[test]
    fn test_smearor_rotation_deg270() {
        let rotation = SmearorRotation::Deg270;
        assert_eq!(rotation.to_degrees(), 270.0);
    }

    #[test]
    fn test_smearor_rotation_custom() {
        let rotation = SmearorRotation::Deg(45.0);
        assert_eq!(rotation.to_degrees(), 45.0);
    }

    #[test]
    fn test_smearor_rotation_from_str() {
        let rotation = SmearorRotation::from("90");
        assert_eq!(rotation.to_degrees(), 90.0);
    }

    #[test]
    fn test_smearor_rotation_from_str_deg() {
        let rotation = SmearorRotation::from("deg180");
        assert_eq!(rotation.to_degrees(), 180.0);
    }

    #[test]
    fn test_smearor_rotation_from_str_custom() {
        let rotation = SmearorRotation::from("45");
        assert_eq!(rotation.to_degrees(), 45.0);
    }

    #[test]
    fn test_rotated_layout_rotation_initial() {
        let layout = imp::RotatedLayout::default();
        assert_eq!(layout.rotation.get(), 0.0);
    }

    #[test]
    fn test_rotated_layout_rotation_set() {
        let layout = imp::RotatedLayout::default();
        layout.rotation.set(90.0);
        assert_eq!(layout.rotation.get(), 90.0);
    }
}
