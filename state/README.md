# State Layer

The State Layer manages the **states** of the application. It ensures that mutable data is available in a thread-safe and synchronized manner.

## Characteristics of this Layer

* **No active logic:** States do not contain render pipelines, file system IO, or event loops.
* **Thread safety:** Uses primitive atomic types (`AtomicBool`, `AtomicU32`, `AtomicF32`) or synchronization objects (`RwLock`, `Mutex`).
* **State Managers:** Each state is accompanied by a state manager (e.g., `MarginManager`, `DebugOverlayStateManager`) that grants thread-safe access via
  defined accessor traits.
* **Dependencies:** Uses types from the **Model Layer**.

---

## List of States and Their Content

### 1. CompositorState (`state/compositor`)

Defines the core state of the Smithay compositor interface.

* **`double_buffer: AtomicBool`:** Determines whether double buffering is enabled for Wayland clients.
* **`dma_buf: AtomicBool`:** Determines whether hardware acceleration via DMA-BUF is enabled.
* **`client_decorations: AtomicBool`:** Determines whether client-side window decorations are allowed.
* **`opacity: AtomicF32`:** Manages the opacity (transparency) of the compositor (0.0 to 1.0).

### 2. DebugOverlayState (`state/debug-overlay`)

State for visual debugging tools.

* **`debug_pointer: AtomicBool`:** Indicates whether the mouse pointer is highlighted visually in snapshots.
* **`debug_touch: AtomicBool`:** Indicates whether touch points are highlighted visually.

### 3. KeyboardState (`state/keyboard`)

Manages the keyboard configuration of the compositor seat.

* **`keyboard_layout: Arc<RwLock<Option<String>>>`:** The active keyboard layout (e.g., `"us"`, `"de"`).
* **`keyboard_variant: Arc<RwLock<Option<String>>>`:** The keyboard variant (e.g., `"nodeadkeys"`).

### 4. LayerShellState (`state/layer-shell`)

Controls the positioning and classification of the compositor window in Wayland environments via layer-shell.

* **`layer: Option<SmearorLayer>`:** Determines which layer the window is placed on (e.g., Background, Bottom, Top, Overlay).
* **`namespace: Option<String>`:** Namespace string to identify the layer-shell window.

### 5. MarginState (`state/margin`)

Manages the margins between the outer GTK window and the inner Wayland windows.

* **`margin_left: AtomicU32`:** Left margin in pixels.
* **`margin_right: AtomicU32`:** Right margin in pixels.
* **`margin_top: AtomicU32`:** Top margin in pixels.
* **`margin_bottom: AtomicU32`:** Bottom margin in pixels.
* **`dialog_margin: AtomicU32`:** Margin for dialog windows (modal dialogs).

### 6. WindowState (`state/window`)

Describes the state of the (outer) GTK application window.

* **`aspect_ratio: Option<AtomicF32>`:** Aspect ratio constraint (width / height).
* **`fullscreen: AtomicBool`:** Flag for fullscreen mode.
* **`initial_position: Option<Position<i32>>`:** Optional initial window coordinates.
* **`initial_size: Option<Size<i32>>`:** Optional initial window dimensions.
* **`max_size: Option<Size<i32>>`:** Maximum window size.
* **`maximized: AtomicBool`:** Determines whether the window starts maximized.
* **`min_size: Option<Size<i32>>`:** Minimum window size.
