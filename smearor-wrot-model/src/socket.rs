use std::ffi::OsStr;
use std::fmt::Display;
use std::fmt::Formatter;
use std::ops::Deref;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Socket(pub PathBuf);

impl Deref for Socket {
    type Target = Path;

    fn deref(&self) -> &Path {
        self.0.as_path()
    }
}

impl AsRef<OsStr> for Socket {
    fn as_ref(&self) -> &OsStr {
        self.0.as_os_str()
    }
}

impl AsRef<str> for Socket {
    fn as_ref(&self) -> &str {
        self.0.as_os_str().to_str().unwrap_or("")
    }
}

impl Display for Socket {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.display())
    }
}

impl From<PathBuf> for Socket {
    fn from(path: PathBuf) -> Self {
        Self(path)
    }
}
