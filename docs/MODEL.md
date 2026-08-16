# smearor-wrot-model

## Overview

`smearor-wrot-model` is the centralized data model crate for the `smearor-wrot` workspace. It defines all shared types used across the compositor, GTK widget, application, pie menu, and wrapper crates. The crate is designed to be lightweight with no mandatory external dependencies — all framework integrations (GTK4, Smithay, image, serde) are optional via feature flags.

## Module Structure

| Module | Types | Description |
|--------|-------|-------------|
| `buffer` | `BufferMetadata` | Wayland buffer dimensions and stride metadata |
| `color` | `RgbColor`, `RgbColor24`, `RgbaColor`, `RgbaColor24`, `ColorMask`, `ColorFrequency` | Color types with hex parsing, conversions, and color masking |
| `config` | `DebugOverlayConfig`, `Config`, `WindowConfig`, `CompositorConfig`, `ConfigError` | Configuration structs for TOML-based config files |
| `geometry` | `Position<T>`, `Size<T>`, `Rectangle<T>` | Generic geometry primitives with arithmetic and conversions |
| `keyboard` | `KeyboardLayout` | Keyboard layout detection and representation |
| `margin` | `Margins` | Margin container for widget spacing |
| `menu` | `MenuItem` | Pie menu item with builder pattern, colors as `RgbaColor` |
| `message` | `CompositorMessage` | Messages sent from compositor core to GTK wrapper |
| `pointer` | `PointerPosition<T>` | Pointer/touch position tracking with debug overlay rendering |
| `socket` | `Socket` | Wayland socket path wrapper |
| `texture` | `PixelData<F>`, `BGRA`, `RGBA`, `PixelDataFormat` | Raw pixel data with format-specific conversions |

## Type Reference

### Color Types

#### `RgbColor`
- **Fields**: `red: f32`, `green: f32`, `blue: f32` (0.0–1.0)
- **Conversions**: `From<RgbColor24>`, `From<ColorMask>`, hex parsing via `FromStr`
- **Feature gates**: `From<smithay::utils::...>` (smithay)

#### `RgbaColor`
- **Fields**: `color: RgbColor`, `alpha: f32`
- **Conversions**: `From<RgbColor>` (alpha=1.0), `From<&str>` (hex), `From<String>`, `FromStr`, `From<RgbaColor24>`
- **Feature gates**: `From<gdk::RGBA>` / `Into<gdk::RGBA>` (gtk4)
- **Builder**: `TypedBuilder` with `setter(into)` accepting hex strings, `RgbColor`, or `RgbaColor`

#### `ColorMask`
- **Fields**: `color: RgbColor`, `tolerance: f32`
- **Conversions**: `From<ColorMask>` for `RgbColor`, hex parsing via `ToHex`
- **Methods**: `new()`, `with_default_tolerance()`, `clamp()`, `color()`, `tolerance()`

### Geometry Types

#### `Position<T>`
- **Fields**: `x: T`, `y: T`
- **Conversions**: `From<(T, T)>`, type conversions (`i32`→`f32`, `i32`→`u32`, `f64`→`f32`)
- **Feature gates**: `From<smithay::utils::Point>` (smithay), `rect()` → `gtk4::graphene::Rect` (gtk4)

#### `Size<T>`
- **Fields**: `width: T`, `height: T`
- **Conversions**: `From<(T, T)>`, type conversions
- **Feature gates**: `From<smithay::utils::Size>` (smithay), `From<gdk::Texture>` (gtk4)

#### `Rectangle<T>`
- **Fields**: `position: Position<T>`, `size: Size<T>`
- **Conversions**: `From<Position<T> + Size<T>>`
- **Feature gates**: `From<smithay::utils::Rectangle>` (smithay), `From<gtk4::graphene::Rect>` (gtk4)

### Buffer & Texture Types

#### `BufferMetadata`
- **Fields**: `width: i32`, `height: i32`, `stride: i32`
- **Feature gates**: `From<&smithay::wayland::shm::BufferData>` (smithay)

#### `PixelData<F>`
- **Formats**: `BGRA`, `RGBA` (marker types)
- **Methods**: `new()`, `from_slice()`, `is_zero()`, `into_rgba()`, `to_rgba()`, `replace_color()`, `apply_color_mask()`, `get_frequency_map()`, `get_dominant_color()`
- **Conversions**: `From<&PixelData<BGRA>>` for `PixelData<RGBA>` and vice versa
- **Feature gates**: `save_png()` (image)

### Config Types

#### `DebugOverlayConfig`
- **Fields**: `debug_pointer: bool`, `debug_touch: bool`
- **Always available** (no feature gate)

#### `Config` (serde feature)
- **Fields**: `window: WindowConfig`, `compositor: CompositorConfig`
- **Derives**: `Deserialize`, `Default`

#### `WindowConfig` (serde feature)
- All fields `Option<T>` for partial config files
- **Derives**: `Deserialize`, `Default`

#### `CompositorConfig` (serde feature)
- All fields `Option<T>` for partial config files
- **Derives**: `Deserialize`, `Default`

### Other Types

#### `CompositorMessage`
- **Derives**: `Debug`, `Clone`, `Serialize`, `Deserialize` (serde feature)
- **Variants**: `Resize`, `Maximize`, `Fullscreen`, `Minimize`, `Restore`, `Close`, `Shutdown`, `TitleChanged`, `WindowMapped`, `FirstCommit`, `WaylandSelectionChanged`, `MoveRequest`, `ResizeRequest`

#### `MenuItem`
- **Derives**: `TypedBuilder`, `Hash`, `Eq`, `PartialEq` (by `id`)
- **Color fields**: `label_color: RgbaColor`, `color: RgbaColor` (builder accepts `&str`, `RgbColor`, `RgbaColor` via `Into<RgbaColor>`)

#### `PointerPosition<T>`
- **Fields**: `gtk_pos`, `app_pos`, `size`, `gtk_color`, `app_color`, `border_width`
- **Feature gates**: `new_pointer()`, `new_touch()`, `gtk_rect()`, `app_rect()`, `render_snapshot()` (gtk4)

#### `Socket`
- **Wraps**: `PathBuf`
- **Traits**: `Deref<Target=Path>`, `AsRef<OsStr>`, `AsRef<str>`, `Display`, `From<PathBuf>`

#### `KeyboardLayout`
- **Fields**: `layout: String`, `variant: Option<String>`
- **Methods**: `full_name()`, `from_xkb_rule_names()` (regex feature)

#### `Margins`
- **Fields**: `left`, `right`, `top`, `bottom` (all `u32`)
- **Derives**: `Default`, `TypedBuilder`

## Conversion Matrix

| Source | Target | Trait | Feature |
|--------|--------|-------|---------|
| `RgbColor24` | `RgbColor` | `From` | — |
| `RgbColor` | `RgbColor24` | `From` | — |
| `RgbaColor24` | `RgbaColor` | `From` | — |
| `RgbaColor` | `RgbaColor24` | `From` | — |
| `RgbColor` | `RgbaColor` | `From` | — |
| `&str` (hex) | `RgbaColor` | `From` | — |
| `String` (hex) | `RgbaColor` | `From` | — |
| `ColorMask` | `RgbColor` | `From` | — |
| `RgbaColor` | `gdk::RGBA` | `From` | gtk4 |
| `gdk::RGBA` | `RgbaColor` | `From` | gtk4 |
| `Position<f32>` | `graphene::Rect` | `rect()` | gtk4 |
| `Size<u32>` | `gdk::Texture` | `From` | gtk4 |
| `Rectangle` | `graphene::Rect` | `From` | gtk4 |
| `Position<i32>` | `Point<i32, K>` | `From` | smithay |
| `Point<i32, K>` | `Position<i32>` | `From` | smithay |
| `Size<i32>` | `SSize<i32, K>` | `From` | smithay |
| `Rectangle<i32>` | `SRect<i32, K>` | `From` | smithay |
| `SRect<i32, K>` | `Rectangle<i32>` | `From` | smithay |
| `&BufferData` | `BufferMetadata` | `From` | smithay |
| `PixelData<BGRA>` | `PixelData<RGBA>` | `into_rgba()` / `to_rgba()` / `From` | — |
| `PixelData<RGBA>` | `PixelData<BGRA>` | `From` | — |
| `(&WindowConfig, &CompositorConfig)` | `CompositorWidgetConfig` | `From` | serde |

## Conversion Diagrams

### Color Conversions

```mermaid
graph LR
    subgraph Model["smearor-wrot-model"]
        RgbColor["RgbColor<br/>(f32, f32, f32)"]
        RgbColor24["RgbColor24<br/>(u8, u8, u8)"]
        RgbaColor["RgbaColor<br/>(RgbColor + alpha: f32)"]
        RgbaColor24["RgbaColor24<br/>(RgbColor24 + alpha: u8)"]
        ColorMask["ColorMask<br/>(RgbColor + tolerance: f32)"]
        Hex["&str (Hex)"]
    end

    subgraph GTK["gtk4 (feature-gated)"]
        GdkRGBA["gdk::RGBA"]
    end

    RgbColor -->|"From"| RgbColor24
    RgbColor24 -->|"From"| RgbColor
    RgbaColor -->|"From"| RgbaColor24
    RgbaColor24 -->|"From"| RgbaColor
    RgbaColor -->|"From"| RgbColor
    RgbColor -->|"From"| RgbaColor

    RgbaColor -->|"From (gtk4)"| GdkRGBA
    GdkRGBA -.->|"via .into()"| RgbaColor

    Hex -->|"parse_hex()"| RgbColor
    Hex -->|"parse_hex()"| RgbaColor
    Hex -->|"parse_hex()"| ColorMask
    RgbColor -->|"to_hex()"| Hex
    RgbaColor -->|"to_hex()"| Hex
    ColorMask -->|"to_hex()"| Hex

    ColorMask -->|"color field"| RgbColor
    RgbColor -->|"From"| ColorMask

    style Model fill:#1a1a2e,color:#e0e0e0,stroke:#16213e
    style GTK fill:#2d1b69,color:#e0e0e0,stroke:#5b21b6
```

### Geometry Conversions

```mermaid
graph LR
    subgraph Model["smearor-wrot-model"]
        Position["Position&lt;T&gt;<br/>(x, y)"]
        Size["Size&lt;T&gt;<br/>(width, height)"]
        Rectangle["Rectangle&lt;T&gt;<br/>(Position + Size)"]
        TuplePos["(T, T)"]
        TupleSize["(T, T)"]
    end

    subgraph Smithay["smithay (feature-gated)"]
        SPoint["Point&lt;i32, K&gt;"]
        SSize["Size&lt;i32, K&gt;"]
        SRect["Rectangle&lt;i32, K&gt;"]
    end

    subgraph GTK["gtk4 (feature-gated)"]
        GRect["graphene::Rect"]
        GTexture["gdk::Texture"]
    end

    TuplePos -->|"From"| Position
    TupleSize -->|"From"| Size
    Position -->|"From"| Rectangle
    Size -->|"From"| Rectangle

    Position -->|"From (smithay)"| SPoint
    SPoint -->|"From (smithay)"| Position
    Size -->|"From (smithay)"| SSize
    Rectangle -->|"From (smithay)"| SRect
    SRect -->|"From (smithay)"| Rectangle

    Position -->|"rect() (gtk4)"| GRect
    Size -->|"rect_from_coordinates() (gtk4)"| GRect
    Rectangle -->|"From (gtk4)"| GRect
    GTexture -->|"From (gtk4)"| Size

    Position -->|"i32 → f32"| Position
    Position -->|"i32 → u32"| Position
    Position -->|"f64 → f32"| Position
    Size -->|"i32 → f32"| Size
    Size -->|"i32 → u32"| Size

    style Model fill:#1a1a2e,color:#e0e0e0,stroke:#16213e
    style Smithay fill:#1b3a2d,color:#e0e0e0,stroke:#166534
    style GTK fill:#2d1b69,color:#e0e0e0,stroke:#5b21b6
```

### Buffer & Texture Conversions

```mermaid
graph LR
    subgraph Model["smearor-wrot-model"]
        BufferMetadata["BufferMetadata<br/>(width, height, stride)"]
        PixelDataBGRA["PixelData&lt;BGRA&gt;"]
        PixelDataRGBA["PixelData&lt;RGBA&gt;"]
        PixelDataFormat["PixelDataFormat"]
        Size["Size&lt;u32&gt;"]
    end

    subgraph Smithay["smithay (feature-gated)"]
        BufferData["BufferData"]
    end

    subgraph Image["image crate"]
        ImageBuffer["ImageBuffer&lt;Rgba&gt;"]
        PNG["PNG file"]
    end

    BufferData -->|"From (smithay)"| BufferMetadata
    PixelDataBGRA -->|"into_rgba() / to_rgba()"| PixelDataRGBA
    PixelDataBGRA -->|"From"| PixelDataRGBA
    PixelDataRGBA -->|"From"| PixelDataBGRA
    PixelDataBGRA -->|"format()"| PixelDataFormat
    PixelDataRGBA -->|"format()"| PixelDataFormat
    PixelDataRGBA -->|"save_png()"| ImageBuffer
    ImageBuffer -->|"save()"| PNG
    Size -->|"save_png() param"| PixelDataRGBA

    style Model fill:#1a1a2e,color:#e0e0e0,stroke:#16213e
    style Smithay fill:#1b3a2d,color:#e0e0e0,stroke:#166534
    style Image fill:#3d2b1b,color:#e0e0e0,stroke:#92400e
```

### Crate Dependencies

```mermaid
graph TD
    Model["smearor-wrot-model<br/>(data models + conversions)"]
    Core["smearor-wrot-core<br/>(compositor logic)"]
    GTK["smearor-wrot-gtk<br/>(GTK4 widget)"]
    App["smearor-wrot-application<br/>(application lifecycle)"]
    PieMenu["smearor-wrot-pie-menu<br/>(pie menu widget)"]
    Wrapper["smearor-wrot-wrapper<br/>(CLI + entry point)"]
    TestClient["smearor-wrot-test-client"]

    Model -->|"gtk4 feature"| GTK4["gtk4"]
    Model -->|"smithay feature"| Smithay["smithay"]
    Model -->|"image feature"| ImageCrate["image"]
    Model -->|"serde feature"| SerdeCrate["serde"]

    Core --> Model
    GTK --> Model
    GTK --> Core
    App --> Model
    App --> Core
    PieMenu --> Model
    Wrapper --> Model
    Wrapper --> App
    Wrapper --> GTK
    TestClient --> Core

    style Model fill:#1a1a2e,color:#e0e0e0,stroke:#e94560,stroke-width:3px
    style Core fill:#1b3a2d,color:#e0e0e0,stroke:#166534
    style GTK fill:#2d1b69,color:#e0e0e0,stroke:#5b21b6
    style App fill:#3d2b1b,color:#e0e0e0,stroke:#92400e
    style PieMenu fill:#2d1b3b,color:#e0e0e0,stroke:#7c3aed
    style Wrapper fill:#1a2d3d,color:#e0e0e0,stroke:#0369a1
```

### Data Flow: GTK ↔ Smithay via Model

```mermaid
sequenceDiagram
    participant GTK as GTK4 (smearor-wrot-gtk)
    participant Model as smearor-wrot-model
    participant Smithay as Smithay (smearor-wrot-core)

    Note over GTK,Smithay: Pointer/Mouse Event
    GTK->>Model: gdk::Position → Position<f32>;
    Model->>Smithay: Position<f32>.into() → Point<i32, Logical>;
    Smithay->>Model: Point<i32, Physical> → Position<i32>
    Model->>GTK: Position<i32>.rect() → graphene::Rect

    Note over GTK,Smithay: Color Mask
    GTK->>Model: Hex-String → ColorMask::parse_hex()
    Model->>Smithay: ColorMask → RgbColor (for pixel matching)
    Smithay->>Model: RgbColor24 (dominant color)
    Model->>GTK: RgbaColor.into() → gdk::RGBA

    Note over GTK,Smithay: Buffer/Texture
    Smithay->>Model: BufferData → BufferMetadata
    Smithay->>Model: Raw pixels → PixelData<BGRA>
    Model->>GTK: PixelData → gdk::Texture (via Size)
```

## Feature Flags

| Feature | Description | Dependencies |
|---------|-------------|--------------|
| `gtk4` | GTK4 type conversions (`gdk::RGBA`, `graphene::Rect`, `gdk::Texture`) | `gtk4` |
| `smithay` | Smithay type conversions (`Point`, `Size`, `Rectangle`, `BufferData`) | `smithay` |
| `image` | PNG export via `PixelData::save_png()` | `image` |
| `serde` | Serialize/Deserialize for `Config`, `WindowConfig`, `CompositorConfig`, `CompositorMessage` | `serde` |
| `regex` | Keyboard layout detection from XKB rule names | `regex` |
| `default` | No features enabled by default | — |

## Examples

### Building a MenuItem with colors

```rust
use smearor_wrot_model::menu::MenuItem;
use smearor_wrot_model::color::RgbColor;

// Using hex strings
let item = MenuItem::builder()
    .id(1)
    .label("Close")
    .icon_name("window-close")
    .color("#FF0000")
    .label_color("#FFFFFF")
    .build();

// Using RgbColor
let item = MenuItem::builder()
    .id(2)
    .label("Open")
    .icon_name("document-open")
    .color(RgbColor::new(0.0, 1.0, 0.0))
    .build();
```

### Loading config from TOML

```rust
use smearor_wrot_model::config::Config;

let toml_content = r#"
[window]
title = "My App"
width = 800

[compositor]
opacity = 0.9
"#;
let config: Config = toml::from_str(toml_content)?;
```

### Converting pixel data

```rust
use smearor_wrot_model::texture::{PixelData, BGRA, RGBA};

let bgra = PixelData::<BGRA>::new(vec![10, 20, 30, 255]);
let rgba = bgra.to_rgba();  // copy, original preserved
let rgba2 = bgra.into_rgba(); // consumed, original destroyed
```

## Testing

Tests are inline in each source file under `#[cfg(test)] mod tests`. Feature-gated tests use `#[cfg(feature = "...")]`.

**Run tests:**
```sh
cargo test -p smearor-wrot-model
cargo test -p smearor-wrot-model --all-features
```

**Test coverage:**
- Color: `RgbColor`, `RgbaColor`, `ColorMask` — construction, conversions, hex parsing, clamping
- Geometry: `Position`, `Size`, `Rectangle` — arithmetic, type conversions, defaults, display
- Buffer: `BufferMetadata` — construction, display
- Texture: `PixelData` — empty/filled checks, frequency map, dominant color, BGRA↔RGBA conversion, PNG save
- Message: `CompositorMessage` — variant coverage, clone, debug, serde roundtrip
- Config: `DebugOverlayConfig`, `Config` — defaults, TOML deserialization
- Keyboard: `KeyboardLayout` — construction, display, clone
- Menu: `MenuItem` — builder, radius, hash/eq, colors as `RgbaColor`
- Margin: `Margins` — construction, default, display, builder
- Pointer: `PointerPosition` — construction, `new_pointer`/`new_touch`, `gtk_rect`/`app_rect` (gtk4)
- Socket: `Socket` — `From<PathBuf>`, `Deref`, `AsRef`, `Display`
