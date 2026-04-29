use crate::rotation::SmearorRotation;
use gtk4::graphene::Point;
use gtk4::prelude::*;
use gtk4::subclass::prelude::*;
use gtk4::Orientation;

mod imp {
    use super::*;
    use gtk4::gsk::Transform;
    use std::cell::Cell;
    use std::cmp::Ordering;

    #[derive(Default)]
    pub struct RotatedLayout {
        pub rotation: Cell<f32>,
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
            let angle = self.rotation.get();
            while let Some(ref c) = child {
                if c.should_layout() {
                    let (_, child_nat_w, _, _) = c.measure(Orientation::Horizontal, -1);
                    let (_, child_nat_h, _, _) = c.measure(Orientation::Vertical, -1);
                    let transform = Transform::new()
                        .translate(&Point::new(width as f32 / 2.0, height as f32 / 2.0))
                        .rotate(angle)
                        .translate(&Point::new(child_nat_w as f32 / -2.0, child_nat_h as f32 / -2.0));
                    c.allocate(width, height, baseline, Some(transform));
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
                (0, 0, -1, -1)
            }
        }
    }

    #[derive(Default)]
    pub struct RotatedBox {
        pub child: std::cell::RefCell<Option<gtk4::Widget>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for RotatedBox {
        const NAME: &'static str = "RotatedBox";
        type Type = super::RotationWidget;
        type ParentType = gtk4::Widget;

        fn class_init(klass: &mut Self::Class) {
            klass.set_layout_manager_type::<super::RotatedLayout>();
        }
    }

    impl ObjectImpl for RotatedBox {
        fn constructed(&self) {
            self.parent_constructed();
        }

        fn dispose(&self) {
            if let Some(child) = self.child.borrow_mut().take() {
                child.unparent();
            }
        }
    }
    impl WidgetImpl for RotatedBox {
        fn snapshot(&self, snapshot: &gtk4::Snapshot) {
            let widget = self.obj();
            if let Some(child) = widget.first_child() {
                widget.snapshot_child(&child, snapshot);
            }
        }

        fn size_allocate(&self, width: i32, height: i32, baseline: i32) {
            self.parent_size_allocate(width, height, baseline);
            let widget = self.obj();
            if let Some(surface) = widget.native().and_then(|n| n.surface()) {
                if let Some(child) = widget.first_child() {
                    let angle = widget.layout_manager()
                        .and_then(|lm| lm.downcast::<super::RotatedLayout>().ok())
                        .map(|rl| rl.imp().rotation.get())
                        .unwrap_or(0.0);
                    let (_, child_w, _, _) = child.measure(Orientation::Horizontal, -1);
                    let (_, child_h, _, _) = child.measure(Orientation::Vertical, -1);
                    let region = self.calculate_rotated_region_with_scale(
                        width as f32,
                        height as f32,
                        child_w as f32,
                        child_h as f32,
                        angle
                    );
                    surface.set_input_region(Some(&region));
                }
            }
        }
    }

    impl RotatedBox {
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
                let current_y = y as f32 + 0.5; // Mitte der Zeile
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
                        eprintln!("Failed to union rectangle region: {}", e);
                    }
                }
            }

            region
        }
    }
}

glib::wrapper! {
    pub struct RotationWidget(ObjectSubclass<imp::RotatedBox>)
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
        let layout = self
            .layout_manager()
            .expect("Failed to get layout manager!")
            .downcast::<RotatedLayout>()
            .expect("Layout manager must be RotatedLayout");
        layout.imp().rotation.set(rotation.to_degrees());
        self.queue_allocate();
    }

    pub fn set_child(&self, child: Option<&impl IsA<gtk4::Widget>>) {
        let mut self_child = self.imp().child.borrow_mut();
        if let Some(old_child) = self_child.take() {
            old_child.unparent();
        }
        if let Some(new_child) = child {
            new_child.set_parent(self);
            *self_child = Some(new_child.clone().upcast());
        }
    }
}
