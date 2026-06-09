use crate::ToHex;
use dashmap::DashMap;
use std::cmp::Ordering;
use std::fmt::Display;
use std::fmt::Formatter;
use std::hash::Hash;
use std::ops::Deref;

#[derive(Debug, Clone, Copy, Eq)]
pub struct ColorFrequency<PDF: Clone> {
    pub color: PDF,
    pub frequency: usize,
}
impl<PDF: Clone> ColorFrequency<PDF> {
    pub fn new(color: PDF, frequency: usize) -> Self {
        Self { color, frequency }
    }
}

impl<PDF: Clone> PartialEq for ColorFrequency<PDF> {
    fn eq(&self, other: &Self) -> bool {
        self.frequency == other.frequency
    }
}
impl<PDF: Clone> PartialOrd for ColorFrequency<PDF> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.frequency.cmp(&other.frequency))
    }
}

impl<PDF: Clone + Eq> Ord for ColorFrequency<PDF> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.frequency.cmp(&other.frequency)
    }
}

pub struct ColorFrequencyMap<PDF: Eq + Hash>(pub DashMap<PDF, usize>);

impl<PDF: Clone + Copy + Eq + Hash + ToHex> Default for ColorFrequencyMap<PDF> {
    fn default() -> Self {
        Self::new()
    }
}

impl<PDF: Clone + Copy + Eq + Hash + ToHex> ColorFrequencyMap<PDF> {
    pub fn new() -> Self {
        Self(DashMap::new())
    }

    pub fn get_dominant_colors(&self, n: usize) -> Vec<ColorFrequency<PDF>> {
        self.get_sorted().iter().take(n).cloned().collect()
    }

    pub fn get_dominant_color(&self) -> Option<ColorFrequency<PDF>> {
        self.get_sorted().first().cloned()
    }

    pub fn get_sorted(&self) -> Vec<ColorFrequency<PDF>> {
        let mut color_frequency: Vec<ColorFrequency<PDF>> = self.0.iter().map(|entry| ColorFrequency::<PDF>::new(*entry.key(), *entry.value())).collect();
        // Sort by frequency descending, then by color hex for deterministic tie-breaking
        color_frequency.sort_by(|a, b| match b.frequency.cmp(&a.frequency) {
            Ordering::Equal => a.color.to_hex().cmp(&b.color.to_hex()),
            other => other,
        });
        color_frequency
    }
}

impl<PDF: Eq + Hash> Deref for ColorFrequencyMap<PDF> {
    type Target = DashMap<PDF, usize>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<PDF: Clone + Copy + Eq + Hash + ToHex> Display for ColorFrequencyMap<PDF> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.get_sorted()
            .iter()
            .take(5)
            .enumerate()
            .map(|(i, top_color)| format!("  {}: {} - {} pixels", i + 1, top_color.color.to_hex(), top_color.frequency).to_string())
            .collect::<Vec<String>>()
            .join(", ")
            .fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RgbColor24;

    #[test]
    fn test_color_frequency_new() {
        let color = RgbColor24::new(255, 128, 64);
        let freq = ColorFrequency::new(color, 10);
        assert_eq!(freq.color, color);
        assert_eq!(freq.frequency, 10);
    }

    #[test]
    fn test_color_frequency_eq() {
        let color1 = RgbColor24::new(255, 128, 64);
        let color2 = RgbColor24::new(128, 64, 32);
        let freq1 = ColorFrequency::new(color1, 10);
        let freq2 = ColorFrequency::new(color2, 10);
        assert_eq!(freq1, freq2);
    }

    #[test]
    fn test_color_frequency_ne() {
        let color1 = RgbColor24::new(255, 128, 64);
        let color2 = RgbColor24::new(128, 64, 32);
        let freq1 = ColorFrequency::new(color1, 10);
        let freq2 = ColorFrequency::new(color2, 5);
        assert_ne!(freq1, freq2);
    }

    #[test]
    fn test_color_frequency_partial_ord() {
        let color1 = RgbColor24::new(255, 128, 64);
        let color2 = RgbColor24::new(128, 64, 32);
        let freq1 = ColorFrequency::new(color1, 10);
        let freq2 = ColorFrequency::new(color2, 5);
        assert!(freq1 > freq2);
        assert!(freq2 < freq1);
    }

    #[test]
    fn test_color_frequency_ord() {
        let color1 = RgbColor24::new(255, 128, 64);
        let color2 = RgbColor24::new(128, 64, 32);
        let freq1 = ColorFrequency::new(color1, 10);
        let freq2 = ColorFrequency::new(color2, 5);
        assert_eq!(freq1.cmp(&freq2), std::cmp::Ordering::Greater);
    }

    #[test]
    fn test_color_frequency_map_new() {
        let map: ColorFrequencyMap<RgbColor24> = ColorFrequencyMap::new();
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn test_color_frequency_map_default() {
        let map: ColorFrequencyMap<RgbColor24> = ColorFrequencyMap::default();
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn test_color_frequency_map_insert_and_get() {
        let map: ColorFrequencyMap<RgbColor24> = ColorFrequencyMap::new();
        let color = RgbColor24::new(255, 128, 64);
        map.insert(color, 10);
        assert_eq!(map.get(&color).map(|v| *v), Some(10));
    }

    #[test]
    fn test_color_frequency_map_get_dominant_colors() {
        let map: ColorFrequencyMap<RgbColor24> = ColorFrequencyMap::new();
        map.insert(RgbColor24::new(255, 0, 0), 100);
        map.insert(RgbColor24::new(0, 255, 0), 50);
        map.insert(RgbColor24::new(0, 0, 255), 25);

        let dominant = map.get_dominant_colors(2);
        assert_eq!(dominant.len(), 2);
        assert_eq!(dominant[0].frequency, 100);
        assert_eq!(dominant[1].frequency, 50);
    }

    #[test]
    fn test_color_frequency_map_get_dominant_color() {
        let map: ColorFrequencyMap<RgbColor24> = ColorFrequencyMap::new();
        map.insert(RgbColor24::new(255, 0, 0), 100);
        map.insert(RgbColor24::new(0, 255, 0), 50);

        let dominant = map.get_dominant_color();
        assert!(dominant.is_some());
        assert_eq!(dominant.unwrap().frequency, 100);
    }

    #[test]
    fn test_color_frequency_map_get_dominant_color_empty() {
        let map: ColorFrequencyMap<RgbColor24> = ColorFrequencyMap::new();
        let dominant = map.get_dominant_color();
        assert!(dominant.is_none());
    }

    #[test]
    fn test_color_frequency_map_get_sorted() {
        let map: ColorFrequencyMap<RgbColor24> = ColorFrequencyMap::new();
        map.insert(RgbColor24::new(255, 0, 0), 50);
        map.insert(RgbColor24::new(0, 255, 0), 100);
        map.insert(RgbColor24::new(0, 0, 255), 25);

        let sorted = map.get_sorted();
        assert_eq!(sorted.len(), 3);
        assert_eq!(sorted[0].frequency, 100);
        assert_eq!(sorted[1].frequency, 50);
        assert_eq!(sorted[2].frequency, 25);
    }

    #[test]
    fn test_color_frequency_map_get_sorted_tie_breaking() {
        let map: ColorFrequencyMap<RgbColor24> = ColorFrequencyMap::new();
        map.insert(RgbColor24::new(255, 0, 0), 50);
        map.insert(RgbColor24::new(0, 255, 0), 50);
        map.insert(RgbColor24::new(0, 0, 255), 25);

        let sorted = map.get_sorted();
        assert_eq!(sorted.len(), 3);
        assert_eq!(sorted[0].frequency, 50);
        assert_eq!(sorted[1].frequency, 50);
        // Tie-breaking by hex string should be deterministic
        assert!(sorted[0].color.to_hex() <= sorted[1].color.to_hex());
    }

    #[test]
    fn test_color_frequency_map_deref() {
        let map: ColorFrequencyMap<RgbColor24> = ColorFrequencyMap::new();
        let color = RgbColor24::new(255, 128, 64);
        map.insert(color, 10);

        // Test that Deref allows accessing DashMap methods
        assert_eq!(map.len(), 1);
        assert!(map.contains_key(&color));
    }

    #[test]
    fn test_color_frequency_map_display() {
        let map: ColorFrequencyMap<RgbColor24> = ColorFrequencyMap::new();
        map.insert(RgbColor24::new(255, 0, 0), 100);
        map.insert(RgbColor24::new(0, 255, 0), 50);

        let display = format!("{}", map);
        assert!(display.contains("#FF0000"));
        assert!(display.contains("100 pixels"));
        assert!(display.contains("#00FF00"));
        assert!(display.contains("50 pixels"));
    }

    #[test]
    fn test_color_frequency_map_display_empty() {
        let map: ColorFrequencyMap<RgbColor24> = ColorFrequencyMap::new();
        let display = format!("{}", map);
        assert_eq!(display, "");
    }

    #[test]
    fn test_color_frequency_map_multiple_operations() {
        let map: ColorFrequencyMap<RgbColor24> = ColorFrequencyMap::new();
        let color1 = RgbColor24::new(255, 0, 0);
        let color2 = RgbColor24::new(0, 255, 0);

        map.insert(color1, 10);
        map.insert(color2, 20);

        assert_eq!(map.len(), 2);
        assert_eq!(map.get(&color1).map(|v| *v), Some(10));
        assert_eq!(map.get(&color2).map(|v| *v), Some(20));

        map.insert(color1, 15);
        assert_eq!(map.get(&color1).map(|v| *v), Some(15));

        let dominant = map.get_dominant_color().unwrap();
        assert_eq!(dominant.frequency, 20);
    }
}
