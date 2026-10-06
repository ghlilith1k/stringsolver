// stringsolver, a CLI string characters converter written in Rust.
// Copyright (C) <2026>  <ghlilith1k>

// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 2 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License along
// with this program; if not, write to the Free Software Foundation, Inc.,
// 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.

// DEPENDENCIES
use std::env;

// MAIN
fn main() {
    
    // VARIABLES
    let args: Vec<String> = env::args().collect();
    let mut selected_file: Option<String> = None;
    let mut output_location: Option<String> = None;
    
    // ARGS PARSER
    for arg in &args[1..] {
        let arg = arg.strip_prefix("--").unwrap();
        
        match arg.split_once('=') {
            Some((aarg, vval)) => {
                match aarg {
                    "file" => {
                        selected_file = Some(vval.to_string());
                    }

                    "output" => {
                        output_location = Some(vval.to_string());
                    }

                    _ => {}

                }
            }

            None => {
                match arg {
                    "help" => {
                        println!(
                            "stringsolver commands:\n\
                             \x20 --help                    - prints this message.\n\
                             \x20 --file=FILE               - file to translate.\n\
                             \x20 --output=term/FILE        - where to print file content (term to paste contents in foreground,\n\
                             \x20                             FILE to create a new file with translated content.\n\
                             \x20 --debug                   - prints debug info."
                        );
                    }

                    _ => {}

                }
            }
        }
    }

}