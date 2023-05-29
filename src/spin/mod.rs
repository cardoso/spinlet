pub fn bin_path() -> Result<String, std::env::VarError> {
    std::env::var("SPIN_BIN_PATH")
}

pub fn branch() -> Result<String, std::env::VarError> {
    std::env::var("SPIN_BRANCH")
}

pub fn build_date() -> Result<String, std::env::VarError> {
    std::env::var("SPIN_BUILD_DATE")
}

pub fn commit_date() -> Result<String, std::env::VarError> {
    std::env::var("SPIN_COMMIT_DATE")
}

pub fn commit_sha() -> Result<String, std::env::VarError> {
    std::env::var("SPIN_COMMIT_SHA")
}

pub fn debug() -> Result<String, std::env::VarError> {
    std::env::var("SPIN_DEBUG")
}

pub fn target_triple() -> Result<String, std::env::VarError> {
    std::env::var("SPIN_TARGET_TRIPLE")
}

pub fn version() -> Result<String, std::env::VarError> {
    std::env::var("SPIN_VERSION")
}

pub fn version_major() -> Result<String, std::env::VarError> {
    std::env::var("SPIN_VERSION_MAJOR")
}

pub fn version_minor() -> Result<String, std::env::VarError> {
    std::env::var("SPIN_VERSION_MINOR")
}

pub fn version_patch() -> Result<String, std::env::VarError> {
    std::env::var("SPIN_VERSION_PATCH")
}

pub fn version_pre() -> Result<String, std::env::VarError> {
    std::env::var("SPIN_VERSION_PRE")
}