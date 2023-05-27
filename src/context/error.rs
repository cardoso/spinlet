#[derive(Debug)]
pub enum ContextError {
    Build(wasmtime::Error),
    Io(std::io::Error),
    Toml(toml::de::Error),
}


impl From<wasmtime::Error> for ContextError {
    fn from(value: wasmtime::Error) -> Self {
        ContextError::Build(value)
    }
}

impl From<std::io::Error> for ContextError {
    fn from(value: std::io::Error) -> Self {
        ContextError::Io(value)
    }
}

impl From<toml::de::Error> for ContextError {
    fn from(value: toml::de::Error) -> Self {
        ContextError::Toml(value)
    }
}

impl std::fmt::Display for ContextError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContextError::Build(err) => write!(f, "Build error: {}", err),
            ContextError::Io(err) => write!(f, "IO error: {}", err),
            ContextError::Toml(err) => write!(f, "TOML error: {}", err),
        }
    }
}

impl std::error::Error for ContextError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ContextError::Build(err) => Some(err.root_cause()),
            ContextError::Io(err) => Some(err),
            ContextError::Toml(err) => Some(err),
        }
    }
}