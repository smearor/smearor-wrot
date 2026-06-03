use crate::color::hex::ToHex;
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
