use crate::error::AppError;
use crate::profile::Profile;
use gix::bstr::BStr;
use gix::config::Source;
use std::fs;
use std::path::{Path, PathBuf};

/// Updates a git config file using a `Profile`.
pub fn update_config(path: impl AsRef<Path>, profile: &Profile) -> Result<(), AppError> {
    let config_path = get_config_path(path)?;
    let mut config =
        gix::config::File::from_path_no_includes(config_path.clone(), Source::Local).map_err(|e| AppError::Git(e.to_string()))?;
    config
        .set_raw_value("user.name", BStr::new(&profile.name))
        .map_err(|e| AppError::Git(e.to_string()))?;
    config
        .set_raw_value("user.email", BStr::new(&profile.email))
        .map_err(|e| AppError::Git(e.to_string()))?;

    if let Some(ref signing_key) = profile.signing_key {
        config
            .set_raw_value("user.signingKey", BStr::new(signing_key))
            .map_err(|e| AppError::Git(e.to_string()))?;
    } else if let Ok(mut section) = config.section_mut("user", None) {
        section.remove("signingKey");
    }

    if let Some(ref ssh_command) = profile.ssh_command {
        config
            .set_raw_value("core.sshCommand", BStr::new(ssh_command))
            .map_err(|e| AppError::Git(e.to_string()))?;
    } else if let Ok(mut section) = config.section_mut("core", None) {
        section.remove("sshCommand");
    }
    fs::write(&config_path, config.to_string()).map_err(|e| AppError::File(e.to_string()))
}

/// Gets the path to `.git/config`.
fn get_config_path(path: impl AsRef<Path>) -> Result<PathBuf, AppError> {
    gix::discover(path)
        .map(|repo| repo.git_dir().join("config"))
        .map_err(|e| AppError::Git(e.to_string()))
}
