#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub enum ChildProcessStartType {
    /// Executes the given command
    #[default]
    Direct,
    /// Wraps the given command in a shell (shell scripts starting a wayland application)
    Shell,
    /// Wraps the given command in a terminal emulator (alacritty --command) which makes it possible to run a terminal application in the compositor
    Terminal,
}
