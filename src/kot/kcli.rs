/*##############################################################################
## Author: Shaun Reed                                                         ##
## Legal: All Content (c) 2021 Shaun Reed, all rights reserved                ##
## About: Wrapper for StructOpt crate functionality used by kot               ##
##                                                                            ##
## Contact: shaunrd0@gmail.com  | URL: www.shaunreed.com | GitHub: shaunrd0   ##
##############################################################################*/

use std::path::Path;
use structopt::StructOpt;

// =============================================================================
// STRUCTS
// =============================================================================

// -----------------------------------------------------------------------------

/// Struct to outline behavior and features of kot CLI
#[derive(Debug, StructOpt)]
#[structopt(
    name="kot",
    about="CLI for managing Linux user configurations"
)]
pub struct Cli {
    #[structopt(
        help="Local or full path to user configurations to install",
        parse(from_os_str)
    )]
    pub dotfiles_dir: std::path::PathBuf,

    #[structopt(
        help="The location to attempt installation of user configurations",
        default_value="dry-runs/kapper", // TODO: Remove temp default value after tests
        // env = "HOME", // Default value to env variable $HOME
        name="install-dir",
        short, long,
        parse(from_os_str)
    )]
    pub install_dir: std::path::PathBuf,

    #[structopt(
        help="The location to store backups for this user",
        default_value="backups/kapper",
        name="backup-dir",
        short, long,
        parse(from_os_str)
    )]
    pub backup_dir: std::path::PathBuf,

    #[structopt(
        help="Overwrites existing backups",
        short, long
    )]
    pub force: bool,
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
    pub fn normalize(mut self) -> super::Result<Self> {
        // If the path to the dotfiles doesn't exist, exit with error
        if !&self.dotfiles_dir.exists() {
            panic!("Error: Dotfiles configuration at {:?} does not exist", self.dotfiles_dir);
        }
        self.dotfiles_dir = self.dotfiles_dir.canonicalize()?;

        // If either the install or backup dir don't exist, create them
        std::fs::create_dir_all(&self.install_dir)?;
        self.install_dir = self.install_dir.canonicalize()?;
        std::fs::create_dir_all(&self.backup_dir)?;
        self.backup_dir = self.backup_dir.canonicalize()?;

        // + To enforce the correction when error is encountered
        // Get the number of configs currently in backup directory
        // + An empty backup directory returns a count of 1
        let current_backups = self.backup_dir.read_dir()?.count();
        // If there are files in the backup directory already
        if current_backups > 1 {
            // If the --force flag is not set, warn and abort
            if !self.force {
                panic!("\n  Error: Backups already exist at {:?}\
                \n  Set the --force flag to overwrite configurations stored here" , self.backup_dir)
            }
            // If the --force flag is set, remove backups and create new
            // + Move backups to /tmp/<BACKUP_DIRNAME>
            // + If we encounter an error, we can move these temp files back to args.backup_dir
            // + On success we can delete them since new backups will have been created at args.backup_dir
            let mut options = fs_extra::dir::CopyOptions::new();
            options.copy_inside = true;
            options.overwrite = true;
            let mut temp_path = Path::new("/tmp/").to_path_buf();
            temp_path.push(self.backup_dir.file_name().unwrap());
            // Move the old backups to /tmp/ and create a new empty backup directory
            super::kfs::move_dir(&self.backup_dir, &temp_path, Some(&options))?;
            std::fs::create_dir_all(&self.backup_dir)?;
        }

        Ok(self)
    }
}
