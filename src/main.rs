extern crate image;

use std::env;

use image::png::PNGEncoder;

fn main() {
    let params = get_command_line_params();
}

fn get_command_line_params() -> Vec<String> {
    let mut arguments = Vec::new();
    for argument in env::args() {
        arguments.append(argument.to_string());
    }
    arguments
}
