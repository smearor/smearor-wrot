use crate::ChildProcess;
use crate::ChildProcessConfig;
use crate::ChildProcessStartError;
use dashmap::DashMap;

/// Manages all child processes
pub struct ChildProcessManager {
    /// The list of child processes that are currently running with their start configuration
    child_process: DashMap<ChildProcessConfig, ChildProcess>,
}

impl ChildProcessManager {
    pub fn new() -> Self {
        Self { child_process: DashMap::new() }
    }

    pub fn launch_child_process(&self, config: ChildProcessConfig) -> Result<(), ChildProcessStartError> {
        let child_process = config.launch_child_process()?;
        self.child_process.insert(config, child_process);
        Ok(())
    }
}

impl Default for ChildProcessManager {
    fn default() -> Self {
        Self::new()
    }
}
