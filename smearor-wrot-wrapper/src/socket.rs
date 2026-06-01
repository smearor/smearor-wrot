use std::error::Error;
use std::path::PathBuf;
use tracing::debug;

/// Build the full socket path from a relative name in XDG_RUNTIME_DIR
pub fn build_socket_path(socket_name: &str) -> Result<PathBuf, Box<dyn Error>> {
    let runtime_dir = std::env::var("XDG_RUNTIME_DIR").map_err(|e| format!("XDG_RUNTIME_DIR not set: {}", e))?;

    let socket_path = PathBuf::from(runtime_dir.clone()).join(socket_name);
    debug!("Building socket path: {:?} from XDG_RUNTIME_DIR={}", socket_path, runtime_dir);

    Ok(socket_path)
}

/// Generate a unique socket name by incrementing the number at the end
pub fn generate_unique_socket_name(base_name: &str) -> Result<String, Box<dyn Error>> {
    let runtime_dir = std::env::var("XDG_RUNTIME_DIR").map_err(|e| format!("XDG_RUNTIME_DIR not set: {}", e))?;

    // Start with base name (e.g., "smearor-wrot-0")
    let mut counter = 0;
    let mut socket_name = format!("{}-{}", base_name, counter);

    loop {
        let socket_path = PathBuf::from(&runtime_dir).join(&socket_name);
        if !socket_path.exists() {
            return Ok(socket_name);
        }

        counter += 1;
        socket_name = format!("{}-{}", base_name, counter);

        // Safety limit to prevent infinite loop
        if counter > 10000 {
            return Err(format!("Failed to generate unique socket name after 1000 attempts: {:?}", socket_path)
                .to_string()
                .into());
        }
    }
}

/// Check if a socket exists and return an error if it does
pub fn check_socket_exists(socket_name: &str) -> Result<(), Box<dyn Error>> {
    let runtime_dir = std::env::var("XDG_RUNTIME_DIR").map_err(|e| format!("XDG_RUNTIME_DIR not set: {}", e))?;
    let socket_path = PathBuf::from(runtime_dir).join(socket_name);

    if socket_path.exists() {
        return Err(format!("Socket already exists: {:?}", socket_path).into());
    }

    Ok(())
}
