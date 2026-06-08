use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum ChildProcessStartError {
    #[error("Failed to resolve executable path: {0}")]
    ResolveExecutablePathError(String),
    #[error("Executable {0} not found in PATH")]
    ExecutableNotFoundInPath(String),
    #[error("Failed to spawn child process {0}")]
    SpawnChildProcessError(String),
}
