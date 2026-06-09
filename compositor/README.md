# Compositor Layer

The Compositor Layer forms the heart of the Wayland server. It uses the **Smithay library** to control protocols, window positions, render events, and clients.

## Characteristics of this Layer

* **Smithay Integration:** Implements and delegates Wayland protocols (e.g., XDG-Shell, Viewporter, SHM, DMA-BUF, Seat, Data Device).
* **Window and Surface Management:** Encapsulates the management of main windows (toplevels), popups, subsurfaces, and dialogs.
* **Uses the Service Layer:** Responds to inputs and system events, but delegates logical decisions and state changes to the **Service Layer**.

## Included Components

* **`smearor-wrot-compositor`:** Contains the `SmearorCompositor`, protocol handlers (e.g., in `src/handlers/`), input processing (mouse/keyboard/touch), and
  DMA-BUF/double-buffered hardware rendering.
