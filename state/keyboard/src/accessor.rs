pub trait KeyboardStateAccessor {
    /// Returns the keyboard layout
    fn keyboard_layout(&self) -> Option<String>;

    /// Returns the keyboard variant
    fn keyboard_variant(&self) -> Option<String>;

    /// Sets the keyboard layout
    fn set_keyboard_layout(&self, keyboard_layout: String);

    /// Sets the keyboard variant
    fn set_keyboard_variant(&self, keyboard_variant: String);
}
