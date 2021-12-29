/*##############################################################################
## Author: Shaun Reed                                                         ##
## Legal: All Content (c) 2021 Shaun Reed, all rights reserved                ##
## About: Main entry point for Linux configuration manager kot                ##
##                                                                            ##
## Contact: shaunrd0@gmail.com  | URL: www.shaunreed.com | GitHub: shaunrd0   ##
##############################################################################*/

use std::path::PathBuf;

mod kot;

// =============================================================================
// MAIN ENTRY-POINT
// =============================================================================

// -----------------------------------------------------------------------------

fn main() -> kot::Result<()> {
    // Call augmented kot::cli::from_args() to parse CLI arguments
    let args = kot::kcli::from_args()?;
    // At this point all paths exist and have been converted to absolute paths
    println!("args: {:?}\n", args);

    // Attempt to install the configurations, checking for collisions
    match kot::install_configs(&args) {
        Err(e) => {
            // If there was an error, show the error type and run settings
            println!(
                "Error: {:?}\n+ Configs used: {:?}\n+ Install directory: {:?}\n",
                e, args.dotfiles_dir, args.install_dir
            );

            // If we were forcing a backup and met some error, revert backups to last good state
            // TODO: Isolate this to limit error scope to backup related functions
            if args.force {
                let mut temp_path : PathBuf = kot::kfs::Path::new("/tmp/").to_path_buf();
                temp_path.push(args.backup_dir.file_name().unwrap());
                kot::kfs::move_dir(&temp_path, &args.backup_dir, None)?;
            }
        },
        _ => ()
    }
    // Configurations installed successfully
    Ok(())
}
