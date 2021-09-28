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

fn backup_config(config_path: & PathBuf, backup_dir: & PathBuf) -> super::io::Result<()> {
    let mut backup_path = backup_dir.to_owned();
    backup_path.push(config_path.file_name().unwrap());
    match config_path.is_dir() {
        true => {
            let mut options = dir::CopyOptions::new();
            options.copy_inside = true;
            dir::move_dir(config_path, backup_path, &options)
        },
        false => {
            let options = fs_extra::file::CopyOptions::new();
            fs_extra::file::move_file(config_path, backup_path, &options)
        },
    };
    Ok(())
}

// Initialize and return a HashMap<config_dir, config_install_location>
// Later used to check each install location for conflicts before installing
pub fn get_target_paths(args: & super::cli::Cli) -> super::io::Result<HashMap<PathBuf, PathBuf>> {
    let mut config_map = HashMap::new();

    let mut config_target = args.install_dir.to_owned();
    for config_entry in fs::read_dir(&args.configs_dir)? {
        match config_entry {
            Err(err) => return Err(err),
            Ok(entry) => {
                config_target.push(entry.file_name());

                if config_target.exists() {
                    match super::io::prompt(format!("Configuration already exists: {:?}\nAbort? Enter y/n or Y/N: ", config_target)) {
                        true => return Err(std::io::Error::from(std::io::ErrorKind::AlreadyExists)),//panic!("User abort"),
                        false => backup_config(&config_target, &args.backup_dir).ok(), // TODO: Backup colliding configs
                    };
                };

                config_map.entry(entry.path().to_owned())
                    .or_insert(config_target.to_owned());

                config_target.pop();
            },
        }

    }
    Ok(config_map)
}
