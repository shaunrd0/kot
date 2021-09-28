/*##############################################################################
## Author: Shaun Reed                                                         ##
## Legal: All Content (c) 2021 Shaun Reed, all rights reserved                ##
## About: Wrapper for std::io functionality used by kot                       ##
##                                                                            ##
## Contact: shaunrd0@gmail.com  | URL: www.shaunreed.com | GitHub: shaunrd0   ##
##############################################################################*/

// Allow use of kot::io::Result
pub use std::io::Result;

use std::io;

// =============================================================================
// IMPLEMENTATION
// =============================================================================

// -----------------------------------------------------------------------------

pub fn prompt(msg: String) -> bool {
    println!("{}", msg);
    let mut reply = String::new();
    io::stdin().read_line(&mut reply)
        .expect("Failed to read user input");
    match reply.trim() {
        "y" | "Y" => true,
        "n" | "N" => false,
        _ => prompt("Please enter y/n or Y/N\n".to_owned()),
    }
}
