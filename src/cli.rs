use std::path::{PathBuf, Path};
use clap::Parser;

#[derive(Parser)]
#[command(bin_name = "spin let")]
pub struct Cli {
    /// Found in the .spinlets folder without the .wasm extension)
    #[arg(default_value = "shell")]
    spinlet: String,
    /// Folder to look for spinlets in
    #[arg(short, long, default_value = ".spinlets")]
    dir: PathBuf,
    /// Extension of spinlets
    #[arg(short, long, default_value = "wasm")]
    ext: String,
    /// Workspace to run the spinlet in
    #[arg(short, long, default_value = ".")]
    workspace: PathBuf,
    /// Arguments to pass to the spinlet
    #[arg(last = true)]
    args: Vec<String>,
}

impl Cli {
    pub fn path(&self) -> PathBuf {
        self.dir
            .join(&self.spinlet)
            .with_extension(&self.ext)
    }

    pub fn dir(&self) -> &Path {
        &self.dir
    }

    pub fn spinlet(&self) -> &str {
        &self.spinlet
    }

    pub fn args(&self) -> &[String] {
        &self.args
    }

    pub fn workspace(&self) -> &Path {
        &self.workspace
    }

    pub fn envs(&self) -> Vec<(String, String)> {
        vec![
            ("SPINLET".to_string(), self.spinlet.clone()),
            ("SPINLET_DIR".to_string(), self.dir.to_string_lossy().to_string()),
            ("SPINLET_EXT".to_string(), self.ext.clone()),
            ("SPINLET_WORKSPACE".to_string(), self.workspace.to_string_lossy().to_string()),
        ]
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path() {
        let cli = Cli {
            spinlet: "shell".to_string(),
            dir: PathBuf::from(".spinlets"),
            ext: "wasm".to_string(),
            workspace: PathBuf::from("."),
            args: vec![],
        };
        assert_eq!(
            cli.path(),
            PathBuf::from(".spinlets/shell.wasm")
        );
    }

    #[test]
    fn test_dir() {
        let cli = Cli {
            spinlet: "shell".to_string(),
            dir: PathBuf::from(".spinlets"),
            ext: "wasm".to_string(),
            workspace: PathBuf::from("."),
            args: vec![],
        };
        assert_eq!(
            cli.dir(),
            Path::new(".spinlets")
        );
    }

    #[test]
    fn test_spinlet() {
        let cli = Cli {
            spinlet: "shell".to_string(),
            dir: PathBuf::from(".spinlets"),
            ext: "wasm".to_string(),
            workspace: PathBuf::from("."),
            args: vec![],
        };
        assert_eq!(
            cli.spinlet(),
            "shell"
        );
    }

    #[test]
    fn test_args() {
        let cli = Cli {
            spinlet: "shell".to_string(),
            dir: PathBuf::from(".spinlets"),
            ext: "wasm".to_string(),
            workspace: PathBuf::from("."),
            args: vec!["arg1".to_string(), "arg2".to_string()],
        };
        assert_eq!(
            cli.args(),
            &["arg1", "arg2"]
        );
    }

    #[test]
    fn test_workspace() {
        let cli = Cli {
            spinlet: "shell".to_string(),
            dir: PathBuf::from(".spinlets"),
            ext: "wasm".to_string(),
            workspace: PathBuf::from("."),
            args: vec![],
        };
        assert_eq!(
            cli.workspace(),
            Path::new(".")
        );
    }
}