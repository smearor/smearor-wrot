use crate::geometry::position::Position;
use crate::geometry::size::Size;
#[cfg(feature = "gtk4")]
use gtk4::graphene::Rect;
#[cfg(feature = "smithay")]
use smithay::utils::Rectangle as SmithayRectangle;
#[cfg(feature = "smithay")]
use smithay::utils::Size as SmithaySize;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Sub;
use std::ops::SubAssign;

/// A rectangle defined by a position (top-left corner) and a size.
///
/// Generic over the numeric type `T` to support both integer-based (Smithay)
/// and float-based (GTK) coordinate systems.
#[derive(Debug, Clone, Copy)]
pub struct Rectangle<T: Debug + Clone + Copy> {
    /// The top-left position of the rectangle
    pub position: Position<T>,
    /// The size of the rectangle
    pub size: Size<T>,
}

impl<T: Debug + Clone + Copy> Rectangle<T> {
    /// Creates a new rectangle from a position and size
    pub fn new(position: Position<T>, size: Size<T>) -> Self {
        Self { position, size }
    }

    /// Creates a new rectangle from individual components
    pub fn from_components(x: T, y: T, width: T, height: T) -> Self {
        Self {
            position: Position::new(x, y),
            size: Size::new(width, height),
        }
    }
}

impl Rectangle<f32> {
    /// Rounds to the nearest integer (standard for display coordinates)
    pub fn to_i32_round(&self) -> Rectangle<i32> {
        Rectangle::new(self.position.to_i32_round(), self.size.to_i32_round())
    }

    /// Rounds down (for clipping/bounding boxes, to not exceed any pixels)
    pub fn to_i32_floor(&self) -> Rectangle<i32> {
        Rectangle::new(self.position.to_i32_floor(), self.size.to_i32_floor())
    }

    /// Rounds up (for damage tracking, to capture all affected pixels)
    pub fn to_i32_ceil(&self) -> Rectangle<i32> {
        Rectangle::new(self.position.to_i32_ceil(), self.size.to_i32_ceil())
    }
}

impl Rectangle<i32> {
    /// Lossless conversion from Rectangle<i32> to Rectangle<f32>
    pub fn to_f32(&self) -> Rectangle<f32> {
        Rectangle::new(self.position.to_f32(), self.size.to_f32())
    }
}

impl<T: Copy + Debug + Default> Default for Rectangle<T> {
    fn default() -> Self {
        Self::new(Position::default(), Size::default())
    }
}

impl<T: Copy + Debug + Display> Display for Rectangle<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Rectangle(pos: {}, size: {})", self.position, self.size)
    }
}

impl<T: Copy + Debug + AddAssign> AddAssign for Rectangle<T> {
    fn add_assign(&mut self, other: Self) {
        self.position += other.position;
        self.size += other.size;
    }
}

impl<T: Copy + Debug + Add<Output = T>> Add for Rectangle<T> {
    type Output = Rectangle<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            position: self.position + rhs.position,
            size: self.size + rhs.size,
        }
    }
}

impl<T: Copy + Debug + SubAssign> SubAssign for Rectangle<T> {
    fn sub_assign(&mut self, other: Self) {
        self.position -= other.position;
        self.size -= other.size;
    }
}

impl<T: Copy + Debug + Sub<Output = T>> Sub for Rectangle<T> {
    type Output = Rectangle<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            position: self.position - rhs.position,
            size: self.size - rhs.size,
        }
    }
}

impl<T: Copy + Debug + PartialEq> PartialEq for Rectangle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position && self.size == other.size
    }
}

impl<T: Copy + Debug + PartialEq + Eq> Eq for Rectangle<T> {}

#[cfg(feature = "gtk4")]
impl From<Rectangle<f32>> for Rect {
    fn from(rectangle: Rectangle<f32>) -> Self {
        Rect::new(rectangle.position.x, rectangle.position.y, rectangle.size.width, rectangle.size.height)
    }
}

#[cfg(feature = "gtk4")]
impl From<Rectangle<i32>> for Rect {
    fn from(rectangle: Rectangle<i32>) -> Self {
        Rect::new(
            rectangle.position.x as f32,
            rectangle.position.y as f32,
            rectangle.size.width as f32,
            rectangle.size.height as f32,
        )
    }
}

#[cfg(feature = "smithay")]
impl<K> From<Rectangle<i32>> for SmithayRectangle<i32, K> {
    fn from(rectangle: Rectangle<i32>) -> Self {
        SmithayRectangle::new(rectangle.position.into(), SmithaySize::new(rectangle.size.width, rectangle.size.height))
    }
}

#[cfg(feature = "smithay")]
impl<K> From<SmithayRectangle<i32, K>> for Rectangle<i32> {
    fn from(rectangle: SmithayRectangle<i32, K>) -> Self {
        let smithay_position: smithay::utils::Point<i32, K> = rectangle.loc;
        let smithay_size: smithay::utils::Size<i32, K> = rectangle.size;
        Self::new(smithay_position.into(), Size::new(smithay_size.w, smithay_size.h))
    }
}

#[cfg(feature = "smithay")]
impl<K> From<Rectangle<f32>> for SmithayRectangle<f32, K> {
    fn from(rectangle: Rectangle<f32>) -> Self {
        SmithayRectangle::new(
            smithay::utils::Point::new(rectangle.position.x, rectangle.position.y),
            SmithaySize::new(rectangle.size.width, rectangle.size.height),
        )
    }
}

#[cfg(feature = "smithay")]
impl<K> From<SmithayRectangle<f32, K>> for Rectangle<f32> {
    fn from(rectangle: SmithayRectangle<f32, K>) -> Self {
        Self::new(Position::new(rectangle.loc.x, rectangle.loc.y), Size::new(rectangle.size.w, rectangle.size.h))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rectangle_new() {
        let rect = Rectangle::new(Position::new(10, 20), Size::new(100, 200));
        assert_eq!(rect.position.x, 10);
        assert_eq!(rect.position.y, 20);
        assert_eq!(rect.size.width, 100);
        assert_eq!(rect.size.height, 200);
    }

    #[test]
    fn test_rectangle_from_components() {
        let rect = Rectangle::from_components(10, 20, 100, 200);
        assert_eq!(rect.position.x, 10);
        assert_eq!(rect.position.y, 20);
        assert_eq!(rect.size.width, 100);
        assert_eq!(rect.size.height, 200);
    }

    #[test]
    fn test_rectangle_default() {
        let rect: Rectangle<i32> = Rectangle::default();
        assert_eq!(rect.position.x, 0);
        assert_eq!(rect.position.y, 0);
        assert_eq!(rect.size.width, 0);
        assert_eq!(rect.size.height, 0);
    }

    #[test]
    fn test_rectangle_display() {
        let rect = Rectangle::new(Position::new(10, 20), Size::new(100, 200));
        assert_eq!(format!("{}", rect), "Rectangle(pos: (10,20), size: 100x200)");
    }

    #[test]
    fn test_rectangle_add() {
        let r1 = Rectangle::new(Position::new(10, 20), Size::new(100, 200));
        let r2 = Rectangle::new(Position::new(5, 5), Size::new(50, 50));
        let result = r1 + r2;
        assert_eq!(result.position.x, 15);
        assert_eq!(result.position.y, 25);
        assert_eq!(result.size.width, 150);
        assert_eq!(result.size.height, 250);
    }

    #[test]
    fn test_rectangle_sub() {
        let r1 = Rectangle::new(Position::new(15, 25), Size::new(150, 250));
        let r2 = Rectangle::new(Position::new(5, 5), Size::new(50, 50));
        let result = r1 - r2;
        assert_eq!(result.position.x, 10);
        assert_eq!(result.position.y, 20);
        assert_eq!(result.size.width, 100);
        assert_eq!(result.size.height, 200);
    }

    #[test]
    fn test_rectangle_eq() {
        let r1 = Rectangle::new(Position::new(10, 20), Size::new(100, 200));
        let r2 = Rectangle::new(Position::new(10, 20), Size::new(100, 200));
        let r3 = Rectangle::new(Position::new(10, 20), Size::new(100, 201));
        assert_eq!(r1, r2);
        assert_ne!(r1, r3);
    }

    #[test]
    fn test_f32_to_i32_round() {
        let rect = Rectangle::new(Position::new(10.6, 20.4), Size::new(100.5, 200.5));
        let rounded = rect.to_i32_round();
        assert_eq!(rounded.position.x, 11);
        assert_eq!(rounded.position.y, 20);
        assert_eq!(rounded.size.width, 101);
        assert_eq!(rounded.size.height, 201);
    }

    #[test]
    fn test_f32_to_i32_floor() {
        let rect = Rectangle::new(Position::new(10.9, 20.9), Size::new(100.9, 200.9));
        let floored = rect.to_i32_floor();
        assert_eq!(floored.position.x, 10);
        assert_eq!(floored.position.y, 20);
        assert_eq!(floored.size.width, 100);
        assert_eq!(floored.size.height, 200);
    }

    #[test]
    fn test_f32_to_i32_ceil() {
        let rect = Rectangle::new(Position::new(10.1, 20.1), Size::new(100.1, 200.1));
        let ceiled = rect.to_i32_ceil();
        assert_eq!(ceiled.position.x, 11);
        assert_eq!(ceiled.position.y, 21);
        assert_eq!(ceiled.size.width, 101);
        assert_eq!(ceiled.size.height, 201);
    }

    #[test]
    fn test_i32_to_f32() {
        let rect = Rectangle::new(Position::new(10, 20), Size::new(100, 200));
        let as_f32 = rect.to_f32();
        assert_eq!(as_f32.position.x, 10.0);
        assert_eq!(as_f32.position.y, 20.0);
        assert_eq!(as_f32.size.width, 100.0);
        assert_eq!(as_f32.size.height, 200.0);
    }
}
