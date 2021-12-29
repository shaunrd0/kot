/*##############################################################################
## Author: Shaun Reed                                                         ##
## Legal: All Content (c) 2021 Shaun Reed, all rights reserved                ##
## About: Wrapper for std::fs functionality used by kot                       ##
##                                                                            ##
## Contact: shaunrd0@gmail.com  | URL: www.shaunreed.com | GitHub: shaunrd0   ##
##############################################################################*/

// Allow the use of kot::fs::Path and kot::fs::PathBuf from std::path::
pub use std::path::{Path, PathBuf};
pub use std::collections::HashMap;
pub use fs_extra::dir;

use std::fs;

// =============================================================================
// IMPLEMENTATION
// =============================================================================

// -----------------------------------------------------------------------------

/// Initialize and return a HashMap<config_dir, config_install_location>
/// + Later used to check each install location for conflicts before installing
/// + This function does not create or modify any files or directories
pub fn get_target_paths(args: & super::cli::Cli) -> super::Result<HashMap<PathBuf, PathBuf>> {
    let mut config_map = HashMap::new();

    // Local variable for the installation directory as an absolute path
    let mut config_target = args.install_dir.to_owned();
    // For each file or directory within the dotfiles we're installing
    for config_entry in fs::read_dir(&args.dotfiles_dir)? {
        let entry = config_entry?;
        // Create full path to target config file (or directory) by push onto install path
        config_target.push(entry.file_name());

        // If the entry doesn't already exist, insert it into the config_map
        // + Key is full path to source config from dotfiles repo we're installing
        // + Value is desired full path to config at final install location
        // TODO: If the entry does exist, should there be an exception?
        config_map.entry(entry.path().to_owned())
            .or_insert(config_target.to_owned());

        // Reset config_target to be equal to requested install_dir
        config_target.pop();
    }
    Ok(config_map)
}

/// Checks if any config to install collides with existing files or directories
/// + Returns a count of collisions within Some(), else returns None
pub fn check_collisions(config_map : & HashMap<PathBuf, PathBuf>) -> Option<Vec<PathBuf>> {
    let mut config_conflicts = vec![];
    for (_path, target_config) in config_map.iter() {
        // If the target configuration file or directory already exists
        if target_config.exists() {
            config_conflicts.push(target_config.to_owned());
        }
    }
    if config_conflicts.len() > 0 {
        return Some(config_conflicts)
    }
    return None
}

/// Moves a single file from one location to another; Can be used to rename files
/// + To specify options such as overwrite for the copy operation, a custom CopyOptions can be provided
pub fn move_file(src: & PathBuf, dst: & PathBuf,
                options: Option< & fs_extra::file::CopyOptions>) -> super::Result<()> {
    if options.is_none() {
        // Default CopyOptions for moving files
        let mut options = fs_extra::file::CopyOptions::new();
        options.overwrite = false;
    }
    fs_extra::file::move_file(src, dst, options.unwrap())?;
    Ok(())
}

/// Moves a directory and all of it's contents recursively
/// + To specify options such as overwrite for the copy operation, a custom CopyOptions can be provided
pub fn move_dir(src: & PathBuf, dst: & PathBuf,
                options: Option< & fs_extra::dir::CopyOptions>) -> super::Result<()> {
    if options.is_none() {
        // Default CopyOptions for moving directories
        let mut options = fs_extra::dir::CopyOptions::new();
        options.copy_inside = true;
        options.overwrite = false;
    }
    fs_extra::dir::move_dir(src, dst, options.unwrap())?;
    Ok(())
}
