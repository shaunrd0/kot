/*##############################################################################
## Author: Shaun Reed                                                         ##
## Legal: All Content (c) 2021 Shaun Reed, all rights reserved                ##
## About: Wrapper for std::io functionality used by kot                       ##
##                                                                            ##
## Contact: shaunrd0@gmail.com  | URL: www.shaunreed.com | GitHub: shaunrd0   ##
##############################################################################*/

use std::io;

// =============================================================================
// IMPLEMENTATION
// =============================================================================

// -----------------------------------------------------------------------------

/// Asks user for y/n Y/N input, returns true/false respectively
/// + Prompt output defined by msg parameter String
pub fn prompt(msg: String) -> bool {
  println!("{}", msg);
  let mut reply = String::new();
  io::stdin().read_line(&mut reply)
      .expect("Failed to read user input");
  match reply.trim() {
    "y" | "Y" => true,
    "n" | "N" => false,
    // Handle garbage input
    _ => prompt("Please enter Y/y or N/n\n".to_owned()),
  }
}
