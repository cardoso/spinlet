use super::access::error::AccessError;



#[derive(Debug)]
pub enum ManifestError {
    Build(wasmtime::Error),
    Access(AccessError),
    TomlSerialize(toml::ser::Error),
    TomlDeserialize(toml::de::Error),
    Io(std::io::Error),
}

impl From<wasmtime::Error> for ManifestError {
    fn from(err: wasmtime::Error) -> Self {
        ManifestError::Build(err)
    }
}

impl From<AccessError> for ManifestError {
    fn from(err: AccessError) -> Self {
        ManifestError::Access(err)
    }
}

impl From<toml::de::Error> for ManifestError {
    fn from(err: toml::de::Error) -> Self {
        ManifestError::TomlDeserialize(err)
    }
}

impl From<std::io::Error> for ManifestError {
    fn from(err: std::io::Error) -> Self {
        ManifestError::Io(err)
    }
}

impl From<toml::ser::Error> for ManifestError {
    fn from(err: toml::ser::Error) -> Self {
        ManifestError::TomlSerialize(err)
    }
}

impl std::error::Error for ManifestError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ManifestError::Build(err) => Some(err.root_cause()),
            ManifestError::Access(err) => Some(err),
            ManifestError::TomlSerialize(err) => Some(err),
            ManifestError::TomlDeserialize(err) => Some(err),
            ManifestError::Io(err) => Some(err)
        }
    }
}

impl std::fmt::Display for ManifestError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ManifestError::Build(err) => write!(f, "Build error: {}", err),
            ManifestError::Access(err) => write!(f, "Access error: {}", err),
            ManifestError::TomlSerialize(err) => write!(f, "TOML error: {}", err),
            ManifestError::TomlDeserialize(err) => write!(f, "TOML error: {}", err),
            ManifestError::Io(err) => write!(f, "IO error: {}", err)
        }
    }
}