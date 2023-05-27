#[derive(Debug)]
pub enum AccessError {
    Io(std::io::Error),
    Env(std::env::VarError)
}

impl std::error::Error for AccessError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AccessError::Io(err) => Some(err),
            AccessError::Env(err) => Some(err)
        }
    }

    
}

impl std::fmt::Display for AccessError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AccessError::Io(err) => write!(f, "IO error: {}", err),
            AccessError::Env(err) => write!(f, "Env error: {}", err)
        }
    }
}

impl From<std::io::Error> for AccessError {
    fn from(value: std::io::Error) -> Self {
        AccessError::Io(value)
    }
}

impl From<std::env::VarError> for AccessError {
    fn from(value: std::env::VarError) -> Self {
        AccessError::Env(value)
    }   
}