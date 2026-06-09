# Widget Layer

The Widget Layer is the layer for the **graphical user interface** (GUI). It uses **GTK 4** to display windows, overlays, buttons, and dialogs on the desktop.

## Characteristics of this Layer

* **Purely UI-focused:** Contains only widget classes, GTK signal connections, event controllers, and styling definitions (CSS).
* **Interaction with services:** Calls methods in the **Service Layer** to respond to user interactions.
* **Compositor embedding:** The `CompositorWidget` connects with the **Compositor Layer** to embed Wayland surfaces as native GDK textures or via OpenGL
  snapshots into the GTK user interface.

## Included Components

* **`smearor-wrot-compositor-widget`:** Provides the central `CompositorWidget` which renders Wayland contents and forwards GTK input events to the compositor.
* **`smearor-wrot-pie-menu`:** The pie menu for quick control and navigation.
* **`smearor-wrot-settings`:** The settings dialog (`SettingsManager`) which allows changing options such as filters, color modes, keyboard layouts, or debug
  options at runtime.
