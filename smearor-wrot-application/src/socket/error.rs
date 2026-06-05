use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum SocketBuilderError {
    #[error("Socket already exists")]
    SocketAlreadyExists,
    #[error("XdgRuntimeDir not set")]
    XdgRuntimeDirNotSet,
    #[error("Failed to generate unique socket name")]
    GenerateUniqueSocketNameFailed,
}
