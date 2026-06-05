use crate::geometry::size::Size;
#[cfg(feature = "gtk4")]
use gtk4::graphene::Rect;
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
        Position::<f32> {
            x: position.x as f32,
            y: position.y as f32,
        }
    }
}

impl From<Position<i32>> for Position<u32> {
    fn from(position: Position<i32>) -> Self {
        Position::<u32> {
            x: position.x as u32,
            y: position.y as u32,
        }
    }
}

impl From<Position<u32>> for Position<i32> {
    fn from(position: Position<u32>) -> Self {
        Position::<i32> {
            x: position.x as i32,
            y: position.y as i32,
        }
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
