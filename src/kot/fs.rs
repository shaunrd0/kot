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

use std::fs;
use fs_extra::dir;

// =============================================================================
// IMPLEMENTATION
// =============================================================================

// -----------------------------------------------------------------------------

// Creates a backup of configurations that conflict
// + Backup directory location is specified by CLI --backup-dir
// TODO: Automatically create backup directory
// TODO: .kotignore in dotfiles repo to specify files to not install / backup
// TODO: .kotrc in dotfiles repo or home dir to set backup-dir and install-dir?
fn backup_config(config_path: & PathBuf, backup_dir: & PathBuf) -> super::io::Result<()> {
    let mut backup_path = backup_dir.to_owned();
    backup_path.push(config_path.file_name().unwrap());
    match config_path.is_dir() {
        true => {
            // Copy directory with recursion using fs_extra::dir::move_dir
            let mut options = dir::CopyOptions::new();
            options.copy_inside = true;
            dir::move_dir(config_path, backup_path, &options)
        },
        false => {
            // Copy single configuration file
            let options = fs_extra::file::CopyOptions::new();
            fs_extra::file::move_file(config_path, backup_path, &options)
        },
    }.expect(&format!("Error: Unable to backup config: {:?}", config_path));
    Ok(())
}

// Initialize and return a HashMap<config_dir, config_install_location>
// Later used to check each install location for conflicts before installing
pub fn get_target_paths(args: & super::cli::Cli) -> super::io::Result<HashMap<PathBuf, PathBuf>> {
    let mut config_map = HashMap::new();

    // Local variable for the installation directory as an absolute path
    let mut config_target = args.install_dir.to_owned();
    // For each file or directory within the dotfiles we're installing
    for config_entry in fs::read_dir(&args.dotfiles_dir)? {
        // Match result from reading each item in dotfiles, return error if any
        match config_entry {
            Err(err) => return Err(err),
            Ok(entry) => {
                // Create full path to target config file (or directory) by push onto install path
                config_target.push(entry.file_name());

                // If the target configuration file or directory already exists
                if config_target.exists() {
                    // Ask client if they would like to abort given the config collision
                    let msg = format!("Configuration already exists: {:?}\
                    \nAbort? Enter y/n or Y/N: ", config_target);

                    // If we abort, exit; If we continue, back up the configs
                    match super::io::prompt(msg) {
                        true => return Err(std::io::Error::from(std::io::ErrorKind::AlreadyExists)),
                        false => backup_config(&config_target, &args.backup_dir).ok(),
                    };
                };

                // If the entry doesn't already exist, insert it into the config_map
                // TODO: If the entry does exist, should there be an exception?
                config_map.entry(entry.path().to_owned())
                    .or_insert(config_target.to_owned());

                // Reset config_target to be equal to requested install_dir
                config_target.pop();
            },
        }

    }

    Ok(config_map)
}
