/*##############################################################################
## Author: Shaun Reed                                                         ##
## Legal: All Content (c) 2021 Shaun Reed, all rights reserved                ##
## About: Root module for Linux configuration manager kot                     ##
##                                                                            ##
## Contact: shaunrd0@gmail.com  | URL: www.shaunreed.com | GitHub: shaunrd0   ##
##############################################################################*/

use std::collections::HashMap;
use std::path::PathBuf;

pub mod kcli;
pub mod kfs;
pub mod kio;
pub mod kgit;
pub mod kerror;

use kerror::Error;
/// Result alias to return result with Error of various types
pub type Result<T> = std::result::Result<T, kerror::Error>;

macro_rules! err {
  ($type:expr, $msg:expr) => {
    return Err(Error::new($type, $msg))
  };

  ($msg:expr) => {
    return Err(Error::new(ErrorKind::Other("Unclassified kot error"), $msg))
  };
}
pub (crate) use err;
use crate::ErrorKind::Other;
use crate::kot::kfs::get_target_paths;

// =============================================================================
// IMPLEMENTATION
// =============================================================================

// -----------------------------------------------------------------------------

pub fn handle_args(args: &mut kcli::Cli) -> Result<()> {
  if args.is_repo {
    // Attempt to install dotfiles from a dotfiles repository
    // + No specific configuration required on behalf of dotfiles repo
    kgit::clone(&args.dotfiles.to_str().unwrap(),
                &args.clone_dir.as_ref().unwrap())?;
  }
  return match install_configs(args) {
    Ok(_) => Ok(()),
    Err(e) => {
      // If we reach an error, use our backup_dir to restore configs
      // + Remove configs we applied that weren't previously on the system
      uninstall_configs(args)?;
      Err(e)
    }
  }
}

/// Creates symbolic links to the configurations we're installing
pub fn install_configs(args: &mut kcli::Cli) -> Result<()> {
  //
  // Find path that points us to the dotfiles we are installing
  let dotfiles = match args.is_repo {
    // If the dotfiles were provided as a system path, use it
    false => args.dotfiles.to_owned(),
    // If the dotfiles to install was a repository, find the path we cloned to
    true => args.clone_dir.as_ref().unwrap().to_path_buf()
  };

  //
  // Check if there are any existing files in the install directory that are also within the dotfiles to install

  // Get the configurations and their target installation paths in a hashmap<config, target_path>
  // + Using target_path, check for conflicts and prompts user to abort or continue
  let config_map = kfs::get_target_paths(&args.install_dir, &dotfiles)?;
  handle_collisions(args, &config_map)?;

  //
  // Install the dotfiles configurations

  // At this point there are either no conflicts or the user agreed to them
  println!("Installing configs:");
  for (config_path, target_path) in &config_map {
    println!("  + {:?}", target_path);
    std::os::unix::fs::symlink(config_path, target_path)
        .or_else(|err| -> Result<()> {
          eprintln!("Error: Unable to create symlink {:?} -> {:?} ({:?})",
                    target_path, config_path, err);

          // Attempt to remove the file or directory first, and then symlink the new config
          match target_path.is_dir() {
            true => fs_extra::dir::remove(target_path)
                .expect(&format!("Error: Unable to remove directory: {:?}", target_path)),
            false => fs_extra::file::remove(target_path)
                .expect(&format!("Error: Unable to remove file: {:?}", target_path)),
          };
          // Try to symlink the config again, if failure exit with error
          std::os::unix::fs::symlink(config_path, target_path).or_else(|err| {
            eprintln!("Error: Unable to symlink config: {:?} -> {:?}",
                      target_path, config_path);
            return Err(err);
          })?;

          return Ok(());
        })?;
  }

  return Ok(());
}

/// Handles collisions between existing files and dotfiles we're installing
/// + If --force is not set, prompt user to continue based on conflicts found
/// + If --force is set or user chooses to continue,
///     move conflicting files to a backup directory
fn handle_collisions(args: &mut kcli::Cli,
                     config_map: &kfs::HashMap<PathBuf, PathBuf>) -> Result<()> {
  // Check if we found any collisions in the configurations
  return match check_collisions(&config_map) {
    None => Ok(()), // There were no collisions, configurations pass pre-install checks
    Some(conflicts) => {
      args.conflicts = conflicts.to_owned();
      // Ask client if they would like to abort given the config collisions
      let mut msg = format!("The following configurations already exist:");
      for config in conflicts.iter() {
        msg += format!("\n  {:?}", config).as_str();
      }
      msg += format!("\nIf you continue, backups will be made in {:?}. \
                      Any configurations there will be overwritten.\
                      \nContinue? Enter Y/y or N/n: ",
                     args.backup_dir.as_ref().unwrap()).as_str();

      // If the --force flag is set, short-circuit boolean and skip prompt
      match args.force || kio::prompt(msg) {
        true => {
          // Backup each conflicting config at the install location
          for backup_target in conflicts.iter() {
            backup_config(backup_target, &args)?;
          }
          Ok(())
        },
        false => err!(Other("User aborted installation".to_string()), "Aborted".to_string())
      }
    }
  };
}

/// Checks if any config to install collides with existing files or directories
/// + Returns a list of collisions within Some(), else returns None
pub fn check_collisions(config_map: &HashMap<PathBuf, PathBuf>)
                        -> Option<Vec<PathBuf>> {
  let mut config_conflicts = vec![];
  for (_path, target_config) in config_map.iter() {
    // If the target configuration file or directory already exists
    if target_config.exists() {
      config_conflicts.push(target_config.to_owned());
    }
  }
  if !config_conflicts.is_empty() {
    return Some(config_conflicts);
  }
  return None;
}

// Creates a backup of configurations that conflict
// + Backup directory location is specified by CLI --backup-dir
// TODO: .kotignore in dotfiles repo to specify files to not install / backup
// TODO: .kotrc in dotfiles repo or home dir to set backup-dir and install-dir?
fn backup_config(config_path: &kfs::PathBuf, args: &kcli::Cli) -> Result<()> {
  let mut backup_path = args.backup_dir.as_ref().unwrap().to_owned();

  // Check if the configuration we're backing up is a directory or a single file
  match config_path.is_dir() {
    true => {
      // Copy directory with recursion using move_dir() wrapper function
      let mut options = fs_extra::dir::CopyOptions::new();
      options.copy_inside = true;
      options.overwrite = args.force;
      kfs::move_dir(config_path, &backup_path, Some(&options))?;
    }
    false => {
      backup_path.push(config_path.file_name().unwrap());
      // Copy single configuration file
      kfs::move_file(config_path, &backup_path)?;
    }
  }
  return Ok(());
}

// Loops through dotfiles to restore backup files or delete unused configs
pub fn uninstall_configs(args: &kcli::Cli) -> Result<()> {
  //
  // Replace previous configs we stored in backup_dir
  for config in args.backup_dir.as_ref().unwrap().read_dir()? {
    match config.as_ref().unwrap().path().is_dir() {
      true => {
        let mut options = fs_extra::dir::CopyOptions::new();
        options.copy_inside = true;
        options.overwrite = args.force;
        kfs::move_dir(&config.as_ref().unwrap().path(), &args.install_dir,
                      Some(&options)
        )?;
      },
      false => {
        kfs::move_file(&config.unwrap().path(), &args.install_dir)?;
      }
    };
  }

  //
  // Remove configurations only required by the dotfiles we attempted to install

  // Check each config in the dotfiles we want to uninstall
  let dotfile_path = match args.is_repo {
    true => args.clone_dir.as_ref().unwrap(),
    false => &args.dotfiles
  };

  for dotfile in dotfile_path.read_dir()? {
    let path = dotfile.unwrap().path();
    // If the configuration was not a conflict initially
    //   then we didn't have it before we installed; It is not being used
    if !args.conflicts.contains(&path) {
      let mut unused_config: PathBuf = args.install_dir.to_owned();
      unused_config.push(std::path::Path::new(&path.file_name().unwrap()));
      // Verify the file was already installed before we hit an error
      if !unused_config.exists() {
        continue;
      }

      // Remove the unused config from install_dir
      std::fs::remove_file(unused_config)?;
    }
  }

  return Ok(());
}
