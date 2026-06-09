#[cfg(feature = "gtk4")]
use crate::Size;
#[cfg(feature = "gtk4")]
use gtk4::graphene::Rect;
#[cfg(feature = "smithay")]
use smithay::utils::Point;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Sub;
use std::ops::SubAssign;

#[derive(Debug, Clone, Copy)]
pub struct Position<T: Debug + Clone + Copy> {
    pub x: T,
    pub y: T,
}

impl<T: Debug + Clone + Copy> Position<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl Position<f32> {
    pub fn new_from_u32(x: u32, y: u32) -> Self {
        Self { x: x as f32, y: y as f32 }
    }

    pub fn new_from_i32(x: i32, y: i32) -> Self {
        Self { x: x as f32, y: y as f32 }
    }
}

impl Position<i32> {
    pub fn max(&self, other: &Self) -> Position<i32> {
        Position::new(self.x.max(other.x), self.y.max(other.y))
    }
}

impl From<Position<i32>> for Position<f32> {
    fn from(position: Position<i32>) -> Self {
        Position {
            x: position.x as f32,
            y: position.y as f32,
        }
    }
}

impl From<Position<i32>> for Position<u32> {
    fn from(position: Position<i32>) -> Self {
        Position {
            x: position.x as u32,
            y: position.y as u32,
        }
    }
}

impl From<Position<u32>> for Position<i32> {
    fn from(position: Position<u32>) -> Self {
        Position {
            x: position.x as i32,
            y: position.y as i32,
        }
    }
}

impl From<Position<f64>> for Position<f32> {
    fn from(position: Position<f64>) -> Self {
        Position {
            x: position.x as f32,
            y: position.y as f32,
        }
    }
}

impl<T: Debug + Clone + Copy> From<(T, T)> for Position<T> {
    fn from(position: (T, T)) -> Self {
        let (x, y) = position;
        Position { x, y }
    }
}

impl<T: Copy + Debug + Default> Default for Position<T> {
    fn default() -> Self {
        Self::new(Default::default(), Default::default())
    }
}

impl<T: Copy + Debug + Display> Display for Position<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl<T: Copy + Debug + AddAssign> AddAssign for Position<T> {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl<T: Copy + Debug + Add<Output = T>> Add for Position<T> {
    type Output = Position<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: Copy + Debug + SubAssign> SubAssign for Position<T> {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl<T: Copy + Debug + Sub<Output = T>> Sub for Position<T> {
    type Output = Position<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: Copy + Debug + PartialEq> PartialEq for Position<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl<T: Copy + Debug + PartialEq + Eq> Eq for Position<T> {}

#[cfg(feature = "gtk4")]
impl Position<i32> {
    pub fn rect_from_coordinates(&self, width: i32, height: i32) -> Rect {
        Rect::new(self.x as f32, self.y as f32, width as f32, height as f32)
    }
}

#[cfg(feature = "gtk4")]
impl Position<f32> {
    pub fn rect_from_coordinates(&self, width: f32, height: f32) -> Rect {
        Rect::new(self.x, self.y, width, height)
    }
}

#[cfg(feature = "gtk4")]
impl Position<i32> {
    pub fn rect(&self, size: Size<i32>) -> Rect {
        Rect::new(self.x as f32, self.y as f32, size.width as f32, size.height as f32)
    }
}

#[cfg(feature = "gtk4")]
impl Position<f32> {
    pub fn rect(&self, size: Size<f32>) -> Rect {
        Rect::new(self.x, self.y, size.width, size.height)
    }
}

#[cfg(feature = "smithay")]
impl<K> From<Position<i32>> for Point<i32, K> {
    fn from(position: Position<i32>) -> Self {
        Self::new(position.x, position.y)
    }
}

#[cfg(feature = "smithay")]
impl<K> From<&Position<i32>> for Point<i32, K> {
    fn from(position: &Position<i32>) -> Self {
        Self::new(position.x, position.y)
    }
}

#[cfg(feature = "smithay")]
impl<K> From<Position<f64>> for Point<f64, K> {
    fn from(position: Position<f64>) -> Self {
        Self::new(position.x, position.y)
    }
}

#[cfg(feature = "smithay")]
impl<K> From<&Position<f64>> for Point<f64, K> {
    fn from(position: &Position<f64>) -> Self {
        Self::new(position.x, position.y)
    }
}

#[cfg(feature = "smithay")]
impl<K> From<Point<i32, K>> for Position<i32> {
    fn from(position: Point<i32, K>) -> Self {
        Self::new(position.x, position.y)
    }
}

#[cfg(feature = "smithay")]
impl<K> From<&Point<i32, K>> for Position<i32> {
    fn from(position: &Point<i32, K>) -> Self {
        Self::new(position.x, position.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_new() {
        let pos = Position::new(10, 20);
        assert_eq!(pos.x, 10);
        assert_eq!(pos.y, 20);
    }

    #[test]
    fn test_position_f32_new_from_u32() {
        let pos = Position::<f32>::new_from_u32(100, 200);
        assert_eq!(pos.x, 100.0);
        assert_eq!(pos.y, 200.0);
    }

    #[test]
    fn test_position_f32_new_from_i32() {
        let pos = Position::<f32>::new_from_i32(-50, 100);
        assert_eq!(pos.x, -50.0);
        assert_eq!(pos.y, 100.0);
    }

    #[test]
    fn test_position_i32_max() {
        let pos1 = Position::new(10, 20);
        let pos2 = Position::new(5, 30);
        let result = pos1.max(&pos2);
        assert_eq!(result.x, 10);
        assert_eq!(result.y, 30);
    }

    #[test]
    fn test_position_from_i32_to_f32() {
        let pos_i32 = Position::new(10, 20);
        let pos_f32: Position<f32> = pos_i32.into();
        assert_eq!(pos_f32.x, 10.0);
        assert_eq!(pos_f32.y, 20.0);
    }

    #[test]
    fn test_position_from_i32_to_u32() {
        let pos_i32 = Position::new(10, 20);
        let pos_u32: Position<u32> = pos_i32.into();
        assert_eq!(pos_u32.x, 10);
        assert_eq!(pos_u32.y, 20);
    }

    #[test]
    fn test_position_from_u32_to_i32() {
        let pos_u32 = Position::new(10, 20);
        let pos_i32: Position<i32> = pos_u32.into();
        assert_eq!(pos_i32.x, 10);
        assert_eq!(pos_i32.y, 20);
    }

    #[test]
    fn test_position_from_f64_to_f32() {
        let pos_f64 = Position::new(10.5, 20.7);
        let pos_f32: Position<f32> = pos_f64.into();
        assert!((pos_f32.x - 10.5).abs() < 0.01);
        assert!((pos_f32.y - 20.7).abs() < 0.01);
    }

    #[test]
    fn test_position_from_tuple() {
        let pos: Position<i32> = (10, 20).into();
        assert_eq!(pos.x, 10);
        assert_eq!(pos.y, 20);
    }

    #[test]
    fn test_position_default() {
        let pos: Position<i32> = Position::default();
        assert_eq!(pos.x, 0);
        assert_eq!(pos.y, 0);
    }

    #[test]
    fn test_position_display() {
        let pos = Position::new(10, 20);
        assert_eq!(format!("{}", pos), "(10,20)");
    }

    #[test]
    fn test_position_add() {
        let pos1 = Position::new(10, 20);
        let pos2 = Position::new(5, 15);
        let result = pos1 + pos2;
        assert_eq!(result.x, 15);
        assert_eq!(result.y, 35);
    }

    #[test]
    fn test_position_add_assign() {
        let mut pos = Position::new(10, 20);
        pos += Position::new(5, 15);
        assert_eq!(pos.x, 15);
        assert_eq!(pos.y, 35);
    }

    #[test]
    fn test_position_sub() {
        let pos1 = Position::new(10, 20);
        let pos2 = Position::new(5, 15);
        let result = pos1 - pos2;
        assert_eq!(result.x, 5);
        assert_eq!(result.y, 5);
    }

    #[test]
    fn test_position_sub_assign() {
        let mut pos = Position::new(10, 20);
        pos -= Position::new(5, 15);
        assert_eq!(pos.x, 5);
        assert_eq!(pos.y, 5);
    }

    #[test]
    fn test_position_eq() {
        let pos1 = Position::new(10, 20);
        let pos2 = Position::new(10, 20);
        let pos3 = Position::new(5, 15);
        assert_eq!(pos1, pos2);
        assert_ne!(pos1, pos3);
    }

    #[test]
    fn test_position_clone() {
        let pos1 = Position::new(10, 20);
        let pos2 = pos1;
        assert_eq!(pos1, pos2);
    }

    #[test]
    fn test_position_copy() {
        let pos1 = Position::new(10, 20);
        let pos2 = pos1;
        assert_eq!(pos1.x, 10);
        assert_eq!(pos2.x, 10);
    }

    #[test]
    fn test_position_debug() {
        let pos = Position::new(10, 20);
        let debug_str = format!("{:?}", pos);
        assert!(debug_str.contains("Position"));
    }
}
