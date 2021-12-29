/*##############################################################################
## Author: Shaun Reed                                                         ##
## Legal: All Content (c) 2021 Shaun Reed, all rights reserved                ##
## About: Root module for Linux configuration manager kot                     ##
##                                                                            ##
## Contact: shaunrd0@gmail.com  | URL: www.shaunreed.com | GitHub: shaunrd0   ##
##############################################################################*/

use std::path::PathBuf;
use crate::kot::kfs::check_collisions;

pub mod kcli;
pub mod kfs;
pub mod kio;

/// Result alias to return result with Error of various types
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

// =============================================================================
// IMPLEMENTATION
// =============================================================================

// -----------------------------------------------------------------------------

/// Creates symbolic links to the configurations we're installing
// TODO: On error, revert to last good state
// TODO: User script to execute after installing configs successfully
pub fn install_configs(args: & kcli::Cli) -> Result<()> {
    // Get the configurations and their target installation paths
    // + Checks for conflicts and prompts user to abort or continue
    let config_map = kfs::get_target_paths(&args)?;

    // Check if there are any existing files in the install directory that are also within the dotfiles to install
    handle_collisions(&args, &config_map)?;

    // At this point there are either no conflicts or the user agreed to them
    println!("Installing configs:");
    for (config_path, target_path) in &config_map {
        println!("  + {:?}", target_path);
        match std::os::unix::fs::symlink(config_path, target_path) {
            Ok(()) => (), // Configuration installed successfully
            Err(_e) => {
                // Attempt to remove the file or directory first, and then symlink the new config
                match target_path.is_dir() {
                    true => fs_extra::dir::remove(target_path)
                        .expect(&format!("Error: Unable to remove directory: {:?}", target_path)),
                    false => fs_extra::file::remove(target_path)
                        .expect(&format!("Error: Unable to remove file: {:?}", target_path)),
                };
                // Try to symlink the config again, if failure exit with error
                std::os::unix::fs::symlink(config_path, target_path)?;
            },
        }
    }

    Ok(())
}

/// Handles collisions between existing files and dotfiles we're installing
fn handle_collisions(args : & kcli::Cli,
                     config_map : & kfs::HashMap<PathBuf, PathBuf>) -> Result<()> {
    // Check if we found any collisions in the configurations
    match check_collisions(&config_map) {
        None => {
            return Ok(()) // There were no collisions, configurations pass pre-install checks
        },
        Some(conflicts) => {
            // Ask client if they would like to abort given the config collisions
            let mut msg = format!("The following configurations already exist:");
            for config in conflicts.iter() {
                msg += format!("\n  {:?}", config).as_str();
            }
            msg += format!("\nIf you continue, backups will be made in {:?}. \
                            Any configurations there will be overwritten.\
                            \nAbort? Enter y/n or Y/N: ", &args.backup_dir).as_str();

            // If we abort, exit; If we continue, back up the configs
            // TODO: Group this in with the --force flag?; Or make a new --adopt flag?
            match kio::prompt(msg) {
                true => return Ok(()),
                false => {
                    // Backup each conflicting config at the install location
                    for backup_target in conflicts.iter() {
                        backup_config(backup_target, &args)?;
                    }
                },
            };

        },
    };

    Ok(())
}

// Creates a backup of configurations that conflict
// + Backup directory location is specified by CLI --backup-dir
// TODO: .kotignore in dotfiles repo to specify files to not install / backup
// TODO: .kotrc in dotfiles repo or home dir to set backup-dir and install-dir?
fn backup_config(config_path: & kfs::PathBuf, args: & kcli::Cli) -> Result<()> {
    let mut backup_path = args.backup_dir.to_owned();
    backup_path.push(config_path.file_name().unwrap());

    // Check if the configuration we're backing up is a directory or a single file
    match config_path.is_dir() {
        true => {
            // Copy directory with recursion using move_dir() wrapper function
            let mut options = kfs::dir::CopyOptions::new();
            options.copy_inside = true;
            options.overwrite = args.force;
            if let Err(e) = kfs::move_dir(config_path, &backup_path, Some(&options))
                .map_err(|e| e.into()) {
                return Err(e)
            }
        },
        false => {
            // Copy single configuration file
            let mut options = fs_extra::file::CopyOptions::new();
            options.overwrite = args.force;
            if let Err(e) = kfs::move_file(config_path, &backup_path, Some(&options))
                .map_err(|e| e.into()) {
                return Err(e)
            }
        },
    }
    Ok(())
}

// TODO: Function to uninstall configs.
// + Loops through dotfiles and restore backup files or delete configs
fn _uninstall_configs() -> Result<()> {
    Ok(())
}
