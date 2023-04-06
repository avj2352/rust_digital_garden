use std::path::PathBuf;
use directories::UserDirs;
use color_eyre::eyre::{eyre, Result};


/// get the default garden directory
pub fn get_default_garden_dir() -> Result<PathBuf> {
    let user_dirs = UserDirs::new().ok_or_else(|| {
        eyre!("Could not find home directory")
    })?;
    Ok(user_dirs.home_dir().join(".garden"))
}
