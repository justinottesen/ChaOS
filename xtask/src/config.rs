//! Build configuration, deserialized from `chaos-config.toml`.
//!
//! The TOML file is the single source of truth. The CLI only chooses a
//! verb (`build`/`run`) and may optionally point at an alternate config
//! file; everything else lives here.

use serde::Deserialize;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
pub struct Config {
    pub build: BuildConfig,
    pub qemu: QemuConfig,
}

#[derive(Debug, Deserialize, Default)]
pub struct BuildConfig {
    #[serde(default)]
    pub target: Target,
    #[serde(default)]
    pub profile: Profile,
    /// Where build artifacts go. If omitted, defaults to the repo-root
    /// `build/` dir. A relative path is anchored to the repo root (not the
    /// cwd) by `resolved_build_dir`.
    #[serde(default = "default_build_dir")]
    pub build_dir: PathBuf,
}

impl BuildConfig {
    /// The build directory as an absolute, cwd-independent path.
    ///
    /// Absolute paths are used as-is; relative paths (including the default)
    /// are anchored to the repo root, so the artifact location never depends
    /// on where `cargo xtask` was invoked from.
    pub fn resolved_build_dir(&self) -> PathBuf {
        if self.build_dir.is_absolute() {
            self.build_dir.clone()
        } else {
            repo_root().join(&self.build_dir)
        }
    }
}

fn default_build_dir() -> PathBuf {
    repo_root().join("build")
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct QemuConfig {
    pub memory: String,
    pub cpus: u32,
    pub serial: String,
}

impl Default for QemuConfig {
    fn default() -> Self {
        Self {
            memory: "512M".into(),
            cpus: 1,
            serial: "stdio".into(),
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum Target {
    #[default]
    X86Bios,
}

#[derive(Debug, Clone, Copy, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Profile {
    #[default]
    Debug,
    Release,
}

/// The repository root: the parent of the `xtask/` crate directory.
///
/// `CARGO_MANIFEST_DIR` is baked in at compile time and points at the
/// `xtask/` crate dir, so this is independent of the current working
/// directory. All repo-relative path logic goes through here so the
/// "parent of the manifest dir" assumption lives in exactly one place.
pub fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask crate should have a parent (the repo root)")
        .to_path_buf()
}

/// Resolve a repo-relative path to an absolute one.
pub fn repo_path(rel: &str) -> PathBuf {
    repo_root().join(rel)
}

impl Config {
    /// Resolve `chaos-config.toml` relative to the repo root.
    pub fn default_path() -> PathBuf {
        repo_root().join("chaos-config.toml")
    }

    /// Load and parse a config file. A missing *file* is a hard error
    /// (the committed default should always exist); a missing *field*
    /// within the file is fine and uses its default.
    pub fn load(path: &Path) -> Result<Self, String> {
        let text = std::fs::read_to_string(path)
            .map_err(|e| format!("could not read config at {}: {e}", path.display()))?;
        toml::from_str(&text).map_err(|e| format!("invalid config in {}: {e}", path.display()))
    }
}
