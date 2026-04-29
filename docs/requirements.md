# smearor-wrot Requirements

## Project Context

### Environment
- **Target Platform**: Table-Top Smart Desk (Smearor) with 65" 4K touchscreen by iiyama
- **Operating System**: Hyprland with Wayland
- **Use Case**: Multi-user collaborative workspace where users sit at different sides of the table

### Problem Statement
The smart desk needs to be accessible from all sides. When users sit opposite each other,
one person sees the content upside down. The solution must allow individual window rotation
without rotating the entire screen, enabling multiple users to interact with windows oriented
toward their position.

## Functional Requirements

### Core Functionality
- **Single Window**: The compositor should only allow a single window
- **Window Rotation**: Ability to rotate the window by any angle and common specific angles (0°, 90°, 180°, 270°)
- **Input Transformation**: Mouse and touch input coordinates must be transformed according to window rotation
- **Application Embedding**: Capability to run any Wayland application within rotated windows
- **Real-time Rendering**: Smooth rendering of embedded applications with minimal latency

### Compositor Requirements
- **Wayland Compatibility**: Must work with Hyprland and other Wayland compositors supporting Layer Shell protocol
- **Flexible Window Modes**: Support for both Layer Shell and normal window modes
- **GNOME Compatibility**: Must function on GNOME desktop without Layer Shell protocol support
- **Single Socket Usage**: Wayland socket should be used by only one program by default

### CLI Application Features
- **Window Configuration**:
    - Toggle window decorations
    - Set window size (width and height)
    - Fullscreen mode support
    - Maximized window mode
- **Runtime Configuration**:
    - Custom socket name specification
    - Rotation angle configuration
    - Command-line parameter forwarding to embedded applications

## Non-Functional Requirements

### Compatibility Requirements
- **Wayland Protocol**: Full compliance with Wayland standards
- **GTK4 Integration**: Seamless integration with GTK4 widget system
- **Cross-Desktop**: Support for major Wayland compositors (Hyprland, GNOME, etc.)
- **Application Compatibility**: Ability to run any standard Wayland application

### Usability Requirements
- **Intuitive Interface**: Simple command-line interface for common use cases
- **Visual Quality**: High-quality rendering at 4K resolution
- **Touch Support**: Full touch input support for the smart desk surface
- **Multi-Window**: Support for multiple rotated windows simultaneously

### Reliability Requirements
- **Stability**: No crashes or memory leaks during extended use
- **Error Handling**: Graceful handling of application failures
- **Resource Management**: Proper cleanup of resources when applications terminate
- **Session Management**: Proper handling of desktop session changes

### Performance Requirements
- **Rendering Performance**: Maintain 60 FPS for smooth user experience
- **Memory Efficiency**: Optimize memory usage for embedded applications
- **Input Latency**: Minimize input processing delay (< 16ms)
- **Hardware Acceleration**: Support for DMA-BUF for GPU-accelerated rendering when available

## Success Criteria

### Functional Success
- Users can rotate windows to any angle
- Input coordinates are correctly transformed
- Any Wayland application can be embedded
- Multiple users can interact simultaneously from different positions

### Quality Success
- Smooth 60 FPS rendering performance
- No noticeable input lag
- Stable operation over extended periods
- Intuitive user experience

---

*This document focuses on what the system should do, not how it should be implemented. Implementation details are documented in AGENTS.md.*
