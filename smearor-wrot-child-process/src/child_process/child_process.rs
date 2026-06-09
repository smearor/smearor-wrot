use crate::ChildProcessState;
use crate::Socket;
use std::io::BufRead;
use std::io::BufReader;
use std::process::Child;
use std::thread;
use tracing::info;

#[derive(Debug)]
pub struct ChildProcess {
    /// The child process
    child: Child,

    /// The config of the child process
    config: ChildProcessState,
}

impl ChildProcess {
    pub fn new(mut child: Child, config: ChildProcessState) -> Self {
        Self { child, config }
    }

    pub fn pid(&self) -> u32 {
        self.child.id()
    }

    pub fn socket(&self) -> Socket {
        self.config.socket()
    }

    /// TODO: tokio async
    pub fn follow_stdout(&mut self) {
        if let Some(stdout) = self.child.stdout.take() {
            let reader = BufReader::new(stdout);
            thread::spawn(move || {
                for line in reader.lines().map_while(Result::ok) {
                    info!("[CHILD STDOUT] {}", line);
                }
            });
        }
    }

    /// TODO: tokio async
    pub fn follow_stderr(&mut self) {
        if let Some(stderr) = self.child.stderr.take() {
            let reader = BufReader::new(stderr);
            thread::spawn(move || {
                for line in reader.lines().map_while(Result::ok) {
                    info!("[CHILD STDERR] {}", line);
                }
            });
        }
    }
}
