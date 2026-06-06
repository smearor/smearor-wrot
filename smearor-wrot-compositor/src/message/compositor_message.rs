/// Messages sent from compositor core to GTK wrapper
#[derive(Debug, Clone)]
pub enum CompositorMessage {
    /// Request to maximize the compositor window
    Maximize,
    /// Request to unmaximize the compositor window
    Unmaximize,
    /// Request to minimize the compositor window
    Minimize,
    /// Request to fullscreen the compositor window
    Fullscreen,
    /// Request to unfullscreen the compositor window
    Unfullscreen,
    /// Request to resize the compositor window
    Resize(i32, i32),
    /// Request to shutdown the compositor when all clients are closed
    Shutdown,
    /// Title of the active client window changed
    TitleChanged(String),
    /// Icon of the active client window changed
    AppIdChanged(String),
    /// A window was mapped (for rotation widget to update size)
    WindowMapped,
    /// First commit received from application (to show window)
    FirstCommit,
    /// Wayland selection changed, extract and sync to host clipboard
    WaylandSelectionChanged,
    /// Client requested to move window (drag by title bar)
    MoveRequest(u32),
    /// Client requested to resize window (drag by edge)
    ResizeRequest(u32),
}
