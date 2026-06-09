# Service Layer

The Service Layer encapsulates the **business logic, workflows, and processes** of the application. It provides logical units that are neither bound to a GUI
framework (like GTK) nor initiate a Wayland server directly.

## Characteristics of this Layer

* **Process and logic-centric:** Contains coordinate calculations, algorithms, flow control, and background services.
* **Manipulates states:** Accesses the **State Layer**, reads values, and manipulates states via state managers.
* **Reusable:** Can be called by different other layers (such as the compositor or widget layer).

## Included Services

* **`smearor-wrot-service-debug-overlay`:** Provides the manager (`DebugOverlayManagerImpl`), which coordinates the management of mouse pointer and touch events
  for the debug overlay.
