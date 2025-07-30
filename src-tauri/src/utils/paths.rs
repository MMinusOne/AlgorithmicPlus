use std::{error::Error, path::PathBuf};
use tauri::Manager;
use crate::APP_HANDLE;

pub fn get_app_data_dir() -> Result<PathBuf, Box<dyn Error>> {
    let app_handle = APP_HANDLE.get().ok_or("Couldn't get app handle")?;
    let app_data_dir = app_handle.path().app_data_dir()?;

    return Ok(app_data_dir);
}

pub fn join_app_data_dir(path: &str) -> Result<PathBuf, Box<dyn Error>> {
    let app_data_dir = get_app_data_dir()?;
    let joined_path = app_data_dir.join(path);

    return Ok(joined_path);
}
