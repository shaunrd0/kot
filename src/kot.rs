/*##############################################################################
## Author: Shaun Reed                                                         ##
## Legal: All Content (c) 2021 Shaun Reed, all rights reserved                ##
## About: Root module for Linux configuration manager kot                     ##
##                                                                            ##
## Contact: shaunrd0@gmail.com  | URL: www.shaunreed.com | GitHub: shaunrd0   ##
##############################################################################*/

pub mod cli;
pub mod fs;
pub mod io;

// =============================================================================
// IMPLEMENTATION
// =============================================================================

// -----------------------------------------------------------------------------

pub fn install_configs(args: & cli::Cli) -> std::io::Result<()> {
    // Get the configurations and their target installation paths
    // + Checks for conflicts and prompts user to abort or continue
    let config_map = fs::get_target_paths(&args)?;

    // At this point there are either no conflicts or the user agreed to them
    for (config_path, target_path) in &config_map {
        println!("Installing config: {:?}\n+ At location: {:?}\n", config_path, target_path);

        match std::os::unix::fs::symlink(config_path, target_path) {
            Ok(()) => (),
            Err(_e) => {
                match target_path.is_dir() {
                    true => fs_extra::dir::remove(target_path),
                    false => fs_extra::file::remove(target_path),
                };
                std::os::unix::fs::symlink(config_path, target_path)
                    .expect(&format!("Unable to symlink config: {:?}", config_path));
            },
        }
    }
    Ok(())
}
