use crate::DEFAULT_SOCKET_PREFIX;
use crate::Socket;
use crate::SocketBuilderError;
use crate::XDG_RUNTIME_DIR;
use std::path::PathBuf;

pub struct SocketBuilder;

impl SocketBuilder {
    pub fn build(socket_name: &Option<String>) -> Result<Socket, SocketBuilderError> {
        let socket_name = if let Some(socket) = socket_name {
            Self::check_socket_exists(socket)?;
            socket.clone()
        } else {
            Self::generate_unique_socket_name(DEFAULT_SOCKET_PREFIX)?
        };
        Self::build_socket_path(&socket_name)
    }

    /// Check if a socket exists and return an error if it does
    pub fn check_socket_exists(socket_name: &str) -> Result<(), SocketBuilderError> {
        let runtime_dir = std::env::var("XDG_RUNTIME_DIR").map_err(|_| SocketBuilderError::XdgRuntimeDirNotSet)?;
        let socket_path = PathBuf::from(runtime_dir).join(socket_name);
        if socket_path.exists() {
            return Err(SocketBuilderError::SocketAlreadyExists);
        }
        Ok(())
    }

    /// Generate a unique socket name by incrementing the number at the end
    pub fn generate_unique_socket_name(base_name: &str) -> Result<String, SocketBuilderError> {
        let runtime_dir = std::env::var(XDG_RUNTIME_DIR).map_err(|_| SocketBuilderError::XdgRuntimeDirNotSet)?;
        let mut counter = 0;
        let mut socket_name = format!("{}-{}", base_name, counter);
        loop {
            let socket_path = PathBuf::from(&runtime_dir).join(&socket_name);
            if !socket_path.exists() {
                return Ok(socket_name);
            }
            counter += 1;
            socket_name = format!("{}-{}", base_name, counter);
            if counter > 10000 {
                return Err(SocketBuilderError::GenerateUniqueSocketNameFailed);
            }
        }
    }

    /// Build the full socket path from a relative name in XDG_RUNTIME_DIR
    pub fn build_socket_path(socket_name: &str) -> Result<Socket, SocketBuilderError> {
        Ok(PathBuf::from(std::env::var("XDG_RUNTIME_DIR").map_err(|_| SocketBuilderError::XdgRuntimeDirNotSet)?.clone())
            .join(socket_name)
            .into())
    }
}
