/*##############################################################################
## Author: Shaun Reed                                                         ##
## Legal: All Content (c) 2021 Shaun Reed, all rights reserved                ##
## About: Root module for Linux configuration manager kot                     ##
##                                                                            ##
## Contact: shaunrd0@gmail.com  | URL: www.shaunreed.com | GitHub: shaunrd0   ##
##############################################################################*/

use std::path::PathBuf;
use crate::kot::fs::check_collisions;

pub mod cli;
pub mod fs;
pub mod io;

// =============================================================================
// IMPLEMENTATION
// =============================================================================

// -----------------------------------------------------------------------------

// Creates symbolic links to the configurations we're installing
// TODO: On error, revert to last good state
// TODO: User script to execute after installing configs successfully
// TODO: Function to uninstall configs. Loop through dotfiles and restore backup files or delete configs
pub fn install_configs(args: & cli::Cli) -> std::io::Result<()> {
    // Get the configurations and their target installation paths
    // + Checks for conflicts and prompts user to abort or continue
    let config_map = fs::get_target_paths(&args)?;
    handle_collisions(&args, &config_map)?;

    // At this point there are either no conflicts or the user agreed to them
    println!("Installing configs:");
    for (config_path, target_path) in &config_map {
        println!("  + {:?}", target_path);
        match std::os::unix::fs::symlink(config_path, target_path) {
            Ok(()) => (), // Configuration installed successfully
            Err(_e) => {
                // Attempt to remove the file or directory, and then symlink the new config
                match target_path.is_dir() {
                    true => fs_extra::dir::remove(target_path)
                        .expect(&format!("Error: Unable to remove directory: {:?}", target_path)),
                    false => fs_extra::file::remove(target_path)
                        .expect(&format!("Error: Unable to remove file: {:?}", target_path)),
                };
                // Try to symlink the config again, if failure exit with error
                std::os::unix::fs::symlink(config_path, target_path)
                    .expect(&format!("Unable to symlink config: {:?}", config_path));
            },
        }
    }
    Ok(())
}

fn handle_collisions(args : & cli::Cli,
                     config_map : & fs::HashMap<PathBuf, PathBuf>) -> io::Result<()> {
    let conflicts = check_collisions(&config_map)
        .expect("Error: Failed to check collisions");

    // If we found collisions in the configurations
    if &conflicts.len() > &0 {
        // Ask client if they would like to abort given the config collisions
        let mut msg = format!("The following configurations already exist:");
        for config in conflicts.iter() {
            msg += format!("\n  {:?}", config).as_str();
        }
        msg += format!("\nIf you continue, backups will be made in {:?}. \
        Any configurations there will be overwritten.\
        \nAbort? Enter y/n or Y/N: ", &args.backup_dir).as_str();

        // If we abort, exit; If we continue, back up the configs
        match io::prompt(msg) {
            true => return Err(std::io::Error::from(std::io::ErrorKind::AlreadyExists)),
            false => {
                // Backup each conflicting config at the install location
                for backup_target in conflicts.iter() {
                    backup_config(backup_target, &args.backup_dir)
                        .expect(format!("Error: Unable to backup config: {:?}", backup_target)
                            .as_str())
                }
            },
        };
    };
    Ok(())
}

// Creates a backup of configurations that conflict
// + Backup directory location is specified by CLI --backup-dir
// TODO: Automatically create backup directory
// TODO: .kotignore in dotfiles repo to specify files to not install / backup
// TODO: .kotrc in dotfiles repo or home dir to set backup-dir and install-dir?
fn backup_config(config_path: & fs::PathBuf, backup_dir: & fs::PathBuf) -> io::Result<()> {
    let mut backup_path = backup_dir.to_owned();
    backup_path.push(config_path.file_name().unwrap());
    match config_path.is_dir() {
        true => {
            // Copy directory with recursion using fs_extra::dir::move_dir
            let mut options = fs::dir::CopyOptions::new();
            options.copy_inside = true;
            // TODO: Add a flag to overwrite backups, otherwise warn and abort
            options.overwrite = true;
            fs::dir::move_dir(config_path, backup_path, &options)
        },
        false => {
            // Copy single configuration file
            let mut options = fs_extra::file::CopyOptions::new();
            options.overwrite = true;
            fs_extra::file::move_file(config_path, backup_path, &options)
        },
    }.expect(&format!("Error: Unable to backup config: {:?}", config_path));
    Ok(())
}
