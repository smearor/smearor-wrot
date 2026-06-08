use std::process::Stdio;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub enum ChildProcessStdio {
    /// The child inherits from the corresponding parent descriptor.
    Inherit,
    /// A new pipe should be arranged to connect the parent and child processes.
    Piped,
    /// This stream will be ignored. This is the equivalent of attaching the stream to /dev/null.
    #[default]
    Null,
}

impl From<&ChildProcessStdio> for Stdio {
    fn from(child_process_stdio_type: &ChildProcessStdio) -> Self {
        match child_process_stdio_type {
            ChildProcessStdio::Inherit => Stdio::inherit(),
            ChildProcessStdio::Piped => Stdio::piped(),
            ChildProcessStdio::Null => Stdio::null(),
        }
    }
}
