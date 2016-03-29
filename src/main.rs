extern crate image;
extern crate dungenon;

use std::io;

use image::png::PNGEncoder;

fn main() {
    let mut level = init_level();

    println!("Type help for list of commands.");
    loop {
        println!("Enter command:");
        let mut line = io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

        match line {
            "Dungeon" => {

            },
            "Maze" => {

            },
            "Room" => {

            },
            "Exit" => {break}
        }
    }
}

fn init_level() -> Level {

}
