#[cfg(feature = "gtk4")]
use gtk4::gdk::Texture;
#[cfg(feature = "gtk4")]
use gtk4::graphene::Rect;
#[cfg(feature = "gtk4")]
use gtk4::prelude::TextureExt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Sub;
use std::ops::SubAssign;

#[derive(Debug, Clone, Copy)]
pub struct Size<T: Debug + Clone + Copy> {
    pub width: T,
    pub height: T,
}

impl<T: Debug + Clone + Copy> Size<T> {
    pub fn new(width: T, height: T) -> Self {
        Self { width, height }
    }
}

impl Size<f32> {
    pub fn new_from_u32(width: u32, height: u32) -> Self {
        Self {
            width: width as f32,
            height: height as f32,
        }
    }

    pub fn new_from_i32(width: i32, height: i32) -> Self {
        Self {
            width: width as f32,
            height: height as f32,
        }
    }
}

impl Size<i32> {
    pub fn max(&self, other: &Self) -> Size<i32> {
        Size::new(self.width.max(other.width), self.height.max(other.height))
    }
}

impl From<Size<i32>> for Size<f32> {
    fn from(size: Size<i32>) -> Self {
        Size::<f32> {
            width: size.width as f32,
            height: size.height as f32,
        }
    }
}

impl From<Size<i32>> for Size<u32> {
    fn from(size: Size<i32>) -> Self {
        Size::<u32> {
            width: size.width as u32,
            height: size.height as u32,
        }
    }
}

impl From<Size<u32>> for Size<i32> {
    fn from(size: Size<u32>) -> Self {
        Size::<i32> {
            width: size.width as i32,
            height: size.height as i32,
        }
    }
}

impl<T: Copy + Debug + Default> Default for Size<T> {
    fn default() -> Self {
        Self::new(Default::default(), Default::default())
    }
}

impl<T: Copy + Debug + Display> Display for Size<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x{}", self.width, self.height)
    }
}

impl<T: Copy + Debug + AddAssign> AddAssign for Size<T> {
    fn add_assign(&mut self, other: Self) {
        self.width += other.width;
        self.height += other.height;
    }
}

impl<T: Copy + Debug + Add<Output = T>> Add for Size<T> {
    type Output = Size<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            width: self.width + rhs.width,
            height: self.height + rhs.height,
        }
    }
}

impl<T: Copy + Debug + SubAssign> SubAssign for Size<T> {
    fn sub_assign(&mut self, other: Self) {
        self.width -= other.width;
        self.height -= other.height;
    }
}

impl<T: Copy + Debug + Sub<Output = T>> Sub for Size<T> {
    type Output = Size<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            width: self.width - rhs.width,
            height: self.height - rhs.height,
        }
    }
}

impl<T: Copy + Debug + PartialEq> PartialEq for Size<T> {
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width && self.height == other.height
    }
}

impl<T: Copy + Debug + PartialEq + Eq> Eq for Size<T> {}

#[cfg(feature = "gtk4")]
impl Size<i32> {
    pub fn rect_from_coordinates(&self, x: i32, y: i32) -> Rect {
        Rect::new(x as f32, y as f32, self.width as f32, self.height as f32)
    }
}

#[cfg(feature = "gtk4")]
impl Size<f32> {
    pub fn rect_from_coordinates(&self, x: f32, y: f32) -> Rect {
        Rect::new(x, y, self.width, self.height)
    }
}

#[cfg(feature = "gtk4")]
impl From<&Texture> for Size<i32> {
    fn from(texture: &Texture) -> Self {
        Size::new(texture.width(), texture.height())
    }
}

#[cfg(feature = "smithay")]
impl<K> From<Size<i32>> for smithay::utils::Size<i32, K> {
    fn from(size: Size<i32>) -> Self {
        Self::new(size.width, size.height)
    }
}

#[cfg(feature = "smithay")]
impl<K> From<&Size<i32>> for smithay::utils::Size<i32, K> {
    fn from(size: &Size<i32>) -> Self {
        Self::new(size.width, size.height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size_new() {
        let size = Size::new(100, 200);
        assert_eq!(size.width, 100);
        assert_eq!(size.height, 200);
    }

    #[test]
    fn test_size_f32_new_from_u32() {
        let size = Size::<f32>::new_from_u32(100, 200);
        assert_eq!(size.width, 100.0);
        assert_eq!(size.height, 200.0);
    }

    #[test]
    fn test_size_f32_new_from_i32() {
        let size = Size::<f32>::new_from_i32(-50, 100);
        assert_eq!(size.width, -50.0);
        assert_eq!(size.height, 100.0);
    }

    #[test]
    fn test_size_i32_max() {
        let size1 = Size::new(100, 200);
        let size2 = Size::new(50, 300);
        let result = size1.max(&size2);
        assert_eq!(result.width, 100);
        assert_eq!(result.height, 300);
    }

    #[test]
    fn test_size_from_i32_to_f32() {
        let size_i32 = Size::new(100, 200);
        let size_f32: Size<f32> = size_i32.into();
        assert_eq!(size_f32.width, 100.0);
        assert_eq!(size_f32.height, 200.0);
    }

    #[test]
    fn test_size_from_i32_to_u32() {
        let size_i32 = Size::new(100, 200);
        let size_u32: Size<u32> = size_i32.into();
        assert_eq!(size_u32.width, 100);
        assert_eq!(size_u32.height, 200);
    }

    #[test]
    fn test_size_from_u32_to_i32() {
        let size_u32 = Size::new(100, 200);
        let size_i32: Size<i32> = size_u32.into();
        assert_eq!(size_i32.width, 100);
        assert_eq!(size_i32.height, 200);
    }

    #[test]
    fn test_size_default() {
        let size: Size<i32> = Size::default();
        assert_eq!(size.width, 0);
        assert_eq!(size.height, 0);
    }

    #[test]
    fn test_size_display() {
        let size = Size::new(100, 200);
        assert_eq!(format!("{}", size), "100x200");
    }

    #[test]
    fn test_size_add() {
        let size1 = Size::new(100, 200);
        let size2 = Size::new(50, 150);
        let result = size1 + size2;
        assert_eq!(result.width, 150);
        assert_eq!(result.height, 350);
    }

    #[test]
    fn test_size_add_assign() {
        let mut size = Size::new(100, 200);
        size += Size::new(50, 150);
        assert_eq!(size.width, 150);
        assert_eq!(size.height, 350);
    }

    #[test]
    fn test_size_sub() {
        let size1 = Size::new(100, 200);
        let size2 = Size::new(50, 150);
        let result = size1 - size2;
        assert_eq!(result.width, 50);
        assert_eq!(result.height, 50);
    }

    #[test]
    fn test_size_sub_assign() {
        let mut size = Size::new(100, 200);
        size -= Size::new(50, 150);
        assert_eq!(size.width, 50);
        assert_eq!(size.height, 50);
    }

    #[test]
    fn test_size_eq() {
        let size1 = Size::new(100, 200);
        let size2 = Size::new(100, 200);
        let size3 = Size::new(50, 150);
        assert_eq!(size1, size2);
        assert_ne!(size1, size3);
    }

    #[test]
    fn test_size_clone() {
        let size1 = Size::new(100, 200);
        let size2 = size1;
        assert_eq!(size1, size2);
    }

    #[test]
    fn test_size_copy() {
        let size1 = Size::new(100, 200);
        let size2 = size1;
        assert_eq!(size1.width, 100);
        assert_eq!(size2.width, 100);
    }

    #[test]
    fn test_size_debug() {
        let size = Size::new(100, 200);
        let debug_str = format!("{:?}", size);
        assert!(debug_str.contains("Size"));
    }
}
