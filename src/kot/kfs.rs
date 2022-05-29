/*##############################################################################
## Author: Shaun Reed                                                         ##
## Legal: All Content (c) 2021 Shaun Reed, all rights reserved                ##
## About: Wrapper for std::fs functionality used by kot                       ##
##                                                                            ##
## Contact: shaunrd0@gmail.com  | URL: www.shaunreed.com | GitHub: shaunrd0   ##
##############################################################################*/

pub use std::path::{Path, PathBuf};
pub use std::collections::HashMap;
pub use fs_extra::dir;

use std::fs;
use crate::kot::err;
use crate::kot::kerror::{Error, ErrorKind};

use super::kgit;

// =============================================================================
// IMPLEMENTATION
// =============================================================================

// -----------------------------------------------------------------------------

pub fn abs(dir: &PathBuf) -> super::Result<PathBuf> {
  return match dir.canonicalize() {
    Ok(result) => Ok(result),
    Err(e) => {
      err!(
        ErrorKind::IOError(e.to_string()),
        format!("Unable to canonicalize dir: {:?}", dir)
      );
    }
  };
}

/// Initialize and return a HashMap<config_dir, config_install_location>
/// + Later used to check each install location for conflicts before installing
/// + This function does not create or modify any files or directories
pub fn get_target_paths(install_dir: &PathBuf, dotfiles: &PathBuf)
                        -> super::Result<HashMap<PathBuf, PathBuf>> {
  let mut config_map = HashMap::new();

  // Local variable for the installation directory as an absolute path
  let mut config_target = install_dir.to_owned();
  // For each file or directory within the dotfiles we're installing
  for config_entry in fs::read_dir(&dotfiles)? {
    let entry = config_entry?;
    // Create full path to target config file (or directory) by push onto install path
    config_target.push(entry.file_name());

    // If the entry doesn't already exist, insert it into the config_map
    // + Key is full path to source config from dotfiles repo we're installing
    // + Value is desired full path to config at final install location
    config_map.entry(entry.path().to_owned())
        .or_insert(config_target.to_owned());

    // Reset config_target to be equal to requested install_dir
    config_target.pop();
  }
  return Ok(config_map);
}

/// Moves a single file from one location to another; Can be used to rename files
/// + Overwrites file at the dst location with the src file
/// + To specify options such as overwrite for the copy operation, a custom CopyOptions can be provided
pub fn move_file(src: &PathBuf, dst: &PathBuf) -> super::Result<()> {
  std::fs::copy(src, dst)?;
  std::fs::remove_file(src)?;
  return Ok(());
}

/// Moves a directory and all of it's contents recursively
/// + To specify options such as overwrite for the copy operation, a custom CopyOptions can be provided
/// TODO: Implement this using std::fs to remove fs_extra dependency
pub fn move_dir(src: &PathBuf, dst: &PathBuf,
                options: Option<&fs_extra::dir::CopyOptions>)
                -> super::Result<()> {
  let copy_options = match options {
    Some(opts) => opts.to_owned(),
    None => {
      // Default CopyOptions for moving directories
      let mut opts = fs_extra::dir::CopyOptions::new();
      opts.copy_inside = true;
      opts.overwrite = false;
      opts
    }
  };

  if let Err(e) = fs_extra::dir::move_dir(src, dst, &copy_options) {
    return err!(
        ErrorKind::DirError(e.to_string()),
        format!("Cannot move directory from {:?} to {:?}", src, dst)
    );
  }
  return Ok(());
}

/// Recursively creates a directory
/// Returns a result that contains the absolute path to the new directory
pub fn create_dir_all(dir: &PathBuf) -> super::Result<PathBuf> {
  return match fs::create_dir_all(dir) {
    Ok(_) => {
      Ok(dir.to_owned())
    },
    Err(e) => {
      err!(
        ErrorKind::IOError(e.to_string()),
        format!("Unable to create directory: {:?}", dir)
      )
    }
  };
}

/// Returns the total number of entries within a directory
/// + Returns 1 for empty directories
pub fn dir_entries(dir: &PathBuf) -> super::Result<usize> {
  if !dir.exists() {
    return Ok(0)
  }
  let count = dir.read_dir().and_then(|dir| Ok(dir.count()))?;
  return Ok(count);
}

/// Stash a directory in the temp folder, staging it for deletion
/// + We stash first instead of delete to allow recovery of these files if we run into an error
pub fn stash_dir(dir: &PathBuf) -> super::Result<()> {
  // Get the number of configs currently in backup directory
  // + An empty backup directory returns a count of 1
  if dir_entries(&dir)? > 1 {
    // Move backups to /tmp/<BACKUP_DIRNAME>
    // + If we encounter an error, we can move these temp files back to args.backup_dir
    // + On success we can delete them since new backups will have been created at args.backup_dir
    let mut options = fs_extra::dir::CopyOptions::new();
    options.copy_inside = true;
    options.overwrite = true;
    let mut temp_path = get_temp_dir();
    temp_path.push(dir.file_name().unwrap());
    // Move the old backups to /tmp/ and create a new empty backup directory
    super::kfs::move_dir(&dir, &temp_path, Some(&options))?;
    std::fs::create_dir_all(&dir)?;
  }
  return Ok(());
}

/// Gets the root temp directory used by kot to store expired files as an owned PathBuf
pub fn get_temp_dir() -> PathBuf {
  // Get temp directory from current user environment
  let mut temp = std::env::temp_dir();
  temp.push("kot/expired/");
  return temp;
}

/// Constructs a new PathBuf pointing to the default data directory used by kot
pub fn get_data_dir() -> PathBuf {
  let mut data_dir = std::path::Path::new(env!("HOME")).to_path_buf();
  data_dir.push(".local/share/kot/");
  return data_dir;
}

/// Constructs a new PathBuf pointing to the default clone directory used by kot
pub fn get_repo_path(repo_url: &str) -> PathBuf {
  let mut repo_path = get_data_dir();
  // Store the new dotfiles repo in a subdirectory using it's name
  repo_path.push("dotfiles/".to_owned() + &kgit::repo_name(repo_url) + "/");
  return repo_path;
}
