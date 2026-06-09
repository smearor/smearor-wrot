# Model Layer

This layer contains exclusively **pure data models** and mathematical types (Value Objects). It is a foundation that can be used by all other layers (especially
the State Layer).

## Characteristics of this Layer

* **No business logic:** Contains no mutable workflows or complex operations.
* **No state mutations:** The types are mostly immutable or offer simple mathematical transformations.
* **No external dependencies (except basic types):** Independent of GTK, Smithay, or configuration parsing.

## Included Data Models

* **`smearor-wrot-model-color`:** Contains color definitions like RGB (`RgbColor`), RGBA (`RgbaColor`), frequency transformation, and hex parsing methods (
  `ParseHexError`, `ToHex`).
* **`smearor-wrot-model-geometry`:** Contains types for position and size specifications (`Position`, `Size`), which are used universally in the compositor and
  the UI.
