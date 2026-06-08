use std::fmt::Display;
use std::fmt::Formatter;
use typed_builder::TypedBuilder;

#[derive(Debug, Default, Clone, Copy, PartialEq, TypedBuilder)]
pub struct Margins {
    pub left: u32,
    pub right: u32,
    pub top: u32,
    pub bottom: u32,
}

impl Margins {
    pub fn new(left: u32, right: u32, top: u32, bottom: u32) -> Self {
        Self { left, right, top, bottom }
    }

    pub fn all(margin: u32) -> Self {
        Self {
            left: margin,
            right: margin,
            top: margin,
            bottom: margin,
        }
    }
}

impl Display for Margins {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(left: {}, right: {}, top: {}, bottom: {})", self.left, self.right, self.top, self.bottom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_margins_new() {
        let margins = Margins::new(10, 20, 30, 40);
        assert_eq!(margins.left, 10);
        assert_eq!(margins.right, 20);
        assert_eq!(margins.top, 30);
        assert_eq!(margins.bottom, 40);
    }

    #[test]
    fn test_margins_default() {
        let margins = Margins::default();
        assert_eq!(margins.left, 0);
        assert_eq!(margins.right, 0);
        assert_eq!(margins.top, 0);
        assert_eq!(margins.bottom, 0);
    }

    #[test]
    fn test_margins_display() {
        let margins = Margins::new(10, 20, 30, 40);
        assert_eq!(format!("{}", margins), "(left: 10, right: 20, top: 30, bottom: 40)");
    }

    #[test]
    fn test_margins_clone() {
        let margins1 = Margins::new(10, 20, 30, 40);
        let margins2 = margins1;
        assert_eq!(margins1, margins2);
    }

    #[test]
    fn test_margins_copy() {
        let margins1 = Margins::new(10, 20, 30, 40);
        let margins2 = margins1;
        assert_eq!(margins1.left, 10);
        assert_eq!(margins2.left, 10);
    }

    #[test]
    fn test_margins_debug() {
        let margins = Margins::new(10, 20, 30, 40);
        let debug_str = format!("{:?}", margins);
        assert!(debug_str.contains("Margins"));
    }

    #[test]
    fn test_margins_eq() {
        let margins1 = Margins::new(10, 20, 30, 40);
        let margins2 = Margins::new(10, 20, 30, 40);
        let margins3 = Margins::new(5, 15, 25, 35);
        assert_eq!(margins1, margins2);
        assert_ne!(margins1, margins3);
    }
}
