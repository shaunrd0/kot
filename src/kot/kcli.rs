/*##############################################################################
## Author: Shaun Reed                                                         ##
## Legal: All Content (c) 2021 Shaun Reed, all rights reserved                ##
## About: Wrapper for StructOpt crate functionality used by kot               ##
##                                                                            ##
## Contact: shaunrd0@gmail.com  | URL: www.shaunreed.com | GitHub: shaunrd0   ##
##############################################################################*/

use std::borrow::Borrow;
use std::path::{Path, PathBuf};
use regex::Regex;
use structopt::StructOpt;
use crate::kot::kerror::{Error, ErrorKind};
use crate::kot::err;
use crate::kot::kfs::create_dir_all;

use chrono;
use super::kfs;

// =============================================================================
// STRUCTS
// =============================================================================

// -----------------------------------------------------------------------------

/// CLI for managing Linux user configurations
#[derive(Debug, StructOpt)]
#[structopt(name = "kot")]
pub struct Cli {
  /// Local or full path to user configurations to install. Can also be a git repository.
  ///
  /// System path or repository URL for dotfiles we want to install.
  /// If a path is used, it can either be local to CWD or absolute.
  /// If a URL is used for a dotfiles repository, the repo is cloned into $HOME/.local/shared/kot/dotfiles/
  #[structopt(parse(from_os_str))]
  pub dotfiles: PathBuf,

  /// The location to attempt installation of user configurations
  ///
  /// The desired installation directory for user configurations.
  /// By default this is your $HOME directory
  /// This could optionally point to some other directory to perform a dry run, or the --dry-run flag could be set
  #[structopt(
  env = "HOME", // Default value to env variable $HOME
  name = "install",
  short, long,
  parse(from_os_str)
  )]
  pub install_dir: PathBuf,

  /// The location to store backups for this user
  ///
  /// If no backup-dir is provided, we create one within the default kot data directory:
  /// $HOME/.local/share/kot/backups/
  #[structopt(
  name = "backup-dir",
  short, long,
  parse(from_os_str)
  )]
  pub backup_dir: Option<PathBuf>,

  /// An alternate path to clone a dotfiles repository to
  ///
  /// If the clone-dir option is provided to the CLI, kot will clone the dotfiles repository into this directory.
  /// If clone-dir is not provided, the repository is cloned into $HOME/.local/share/kot/dotfiles
  /// Custom clone-dir will be used literally, and no subdirectory is created to store the cloned repository
  /// For example, clone-dir of $HOME/clonedir for repo named Dotfiles
  /// We will clone into $HOME/clonedir, and NOT $HOME/clonedir/Dotfiles
  /// The default path for cloned repos is $HOME/.local/share/kot/dotfiles/
  #[structopt(
  name = "clone-dir",
  short, long,
  parse(from_os_str)
  )]
  pub clone_dir: Option<PathBuf>,

  /// Overwrites existing backups
  ///
  /// This flag will replace existing backups if during installation we encounter conflicts
  /// and the backup-dir provided already contains previous backups.
  #[structopt(
  name = "force",
  short, long
  )]
  pub force: bool,

  /// Installs configurations to $HOME/.local/shared/kot/dry-runs
  ///
  /// Useful flag to set when testing what an install would do to your home directory.
  /// This is synonymous with setting --install $HOME/.local/shared/kot/dry-runs/$USER.
  /// Subsequent runs with this flag set will not delete the contents of this directory.
  #[structopt(
  name = "dry-run",
  short, long
  )]
  pub dry_run: bool,

  // Indicates if dotfiles is a git repository URL; Not used by CLI directly
  // + Initialized with result of regex pattern matching
  #[structopt(skip)]
  pub is_repo: bool,

  // Not used by CLI, used to uninstall dotfiles when error is hit
  #[structopt(skip)]
  pub conflicts: Vec<PathBuf>,
}

// =============================================================================
// IMPLEMENTATION
// =============================================================================

// -----------------------------------------------------------------------------

/// Augment implementation of from_args to limit scope of StructOpt
/// + Also enforces use of Cli::normalize()
/// + https://docs.rs/structopt/0.3.23/src/structopt/lib.rs.html#1121-1126
pub fn from_args() -> super::Result<Cli> {
  let s = Cli::from_clap(&Cli::clap().get_matches());
  s.normalize()
}

impl Cli {
  /// Helper function to normalize arguments passed to program
  /// + Checks if dotfiles path is a repository URL
  /// + If dotfiles path is not a repo URL, checks the path exists on the system
  /// + Verifies install directory exists
  /// + Verifies backup directory exists and does not already contain backups
  pub fn normalize(mut self) -> super::Result<Self> {
    // Determine if the dotfiles were provided as a github repository URL
    let re_git = Regex::new(
      r"^(([A-Za-z0-9]+@|http(|s)://)|(http(|s)://[A-Za-z0-9]+@))([A-Za-z0-9.]+(:\d+)?)(?::|/)([\d/\w.-]+?)(\.git){1}$"
    );
    self.is_repo = re_git.unwrap().is_match(&self.dotfiles.to_str().unwrap());

    if self.is_repo {
      // If the dotfiles were provided as a repository URL initialize clone_dir
      self.clone_dir = match &self.clone_dir {
        Some(d) => {
          kfs::create_dir_all(d)?;
          Some(kfs::abs(d)?)
        },
        None => Some(kfs::get_repo_path(self.dotfiles.to_str().unwrap()))
      };
    }
    else {
      // If the dotfiles were provided as a path, canonicalize it
      self.dotfiles = kfs::abs(&self.dotfiles)?;
    }

    //
    // If either the install, backup, or clone dir does not exist, create them

    if self.dry_run {
      self.install_dir = Path::new(
        &(env!("HOME").to_owned() + &"/.local/share/kot/dry-runs/" + env!("USER"))
      ).to_path_buf();
    }
    self.install_dir = kfs::create_dir_all(&self.install_dir)?;
    // If the CLI was not provided a backup_dir, use default naming convention
    match self.backup_dir {
      None => {
        let mut backup_dir = kfs::get_data_dir();
        backup_dir.push("backups/");
        backup_dir.push(self.dotfiles.file_name().unwrap().to_str().unwrap().to_owned()
            + ":" + &*chrono::offset::Local::now()
            .format("%Y-%m-%dT%H:%M:%S").to_string()
        );
        self.backup_dir = Some(kfs::create_dir_all(&backup_dir)?);
      }
      Some(dir) => {
        // If a backup_dir was given to CLI, use it instead of default
        self.backup_dir = Some(kfs::create_dir_all(&dir)?);
      }
    }


    //
    // Check if the backup directory provided is empty

    // If there are files and the --force flag is not set, warn and abort
    if !self.force && kfs::dir_entries(&self.backup_dir.as_ref().unwrap())? > 1 {
      return err!(
        ErrorKind::ConfigError(format!("Backups already exist at: {:?}", &self.backup_dir)),
        "Set the --force flag to overwrite configurations stored here".to_owned()
      );
    }
    // If the --force flag is set, stash backup files in /tmp/ and create new
    kfs::stash_dir(&self.backup_dir.as_ref().unwrap())?;

    // Available CLI options pass initial checks; Return them to caller
    return Ok(self);
  }
}
