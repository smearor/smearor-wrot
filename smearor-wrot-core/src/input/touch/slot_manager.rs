use smithay::backend::input::TouchSlot;
use std::collections::HashMap;
use std::sync::Mutex;

/// Manages touch slots for GTK EventSequence to Smithay TouchSlot conversion
pub struct TouchSlotManager {
    next_slot_id: u32,
    sequence_to_slot: HashMap<usize, TouchSlot>,
    slot_to_sequence: HashMap<TouchSlot, usize>,
}

impl TouchSlotManager {
    /// Create a new TouchSlotManager
    pub fn new() -> Self {
        Self {
            next_slot_id: 0,
            sequence_to_slot: HashMap::new(),
            slot_to_sequence: HashMap::new(),
        }
    }

    /// Get or create a TouchSlot for a GTK EventSequence
    pub fn get_slot_for_sequence(&mut self, sequence: usize) -> TouchSlot {
        if let Some(&slot) = self.sequence_to_slot.get(&sequence) {
            return slot;
        }

        let slot = TouchSlot::from(Some(self.next_slot_id));
        self.sequence_to_slot.insert(sequence, slot);
        self.slot_to_sequence.insert(slot, sequence);
        self.next_slot_id += 1;
        slot
    }

    /// Remove a slot when a touch sequence ends
    pub fn remove_slot_for_sequence(&mut self, sequence: usize) {
        if let Some(slot) = self.sequence_to_slot.remove(&sequence) {
            self.slot_to_sequence.remove(&slot);
        }
    }

    /// Get the sequence for a TouchSlot
    pub fn get_sequence_for_slot(&self, slot: TouchSlot) -> Option<usize> {
        self.slot_to_sequence.get(&slot).copied()
    }

    /// Check if a slot exists for a sequence
    pub fn has_slot_for_sequence(&self, sequence: usize) -> bool {
        self.sequence_to_slot.contains_key(&sequence)
    }
}

impl Default for TouchSlotManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Thread-safe wrapper for TouchSlotManager
pub struct ThreadSafeTouchSlotManager {
    inner: Mutex<TouchSlotManager>,
}

impl ThreadSafeTouchSlotManager {
    /// Create a new ThreadSafeTouchSlotManager
    pub fn new() -> Self {
        Self {
            inner: Mutex::new(TouchSlotManager::new()),
        }
    }

    /// Get or create a TouchSlot for a GTK EventSequence
    pub fn get_slot_for_sequence(&self, sequence: usize) -> TouchSlot {
        let mut inner = self.inner.lock().unwrap();
        inner.get_slot_for_sequence(sequence)
    }

    /// Remove a slot when a touch sequence ends
    pub fn remove_slot_for_sequence(&self, sequence: usize) {
        let mut inner = self.inner.lock().unwrap();
        inner.remove_slot_for_sequence(sequence);
    }

    /// Get the sequence for a TouchSlot
    pub fn get_sequence_for_slot(&self, slot: TouchSlot) -> Option<usize> {
        let inner = self.inner.lock().unwrap();
        inner.get_sequence_for_slot(slot)
    }

    /// Check if a slot exists for a sequence
    pub fn has_slot_for_sequence(&self, sequence: usize) -> bool {
        let inner = self.inner.lock().unwrap();
        inner.has_slot_for_sequence(sequence)
    }
}

impl Default for ThreadSafeTouchSlotManager {
    fn default() -> Self {
        Self::new()
    }
}
