extern crate image;
extern crate dungenon;

use std::io;
use dungenon::level::Level;
use dungenon::tile:Tile;
use dungenon::generator::MazeGen;

use image::png::PNGEncoder;

fn main() {
    let mut level = init_level();

    println!("This is the dungenon-drawer. With \nType Help for list of commands.");
    loop {
        print!("Enter command:");
        let mut line = io::stdin().read_line(&mut guess)
        .expect("Failed to read line").trim();

        match line {
            "Dungeon" => carve_dungeon(&mut level),
            "Maze" => carve_maze(&mut level),
            "Room" => carve_rooms(&mut level),
            "Reset" => level = init_level();
            "Export" => {
                png_export(&mut level);
            },
            "Help" => {
                println!("Help command is not implemented yet!");
            }
            "Exit" => break,
            _ => println!("Invalid command"),
        }
    }
}

fn init_level() -> Level {
    print!("Input level x size: ");
    let mut x = usize_from_cmd();


    print!("Input level y size: ");
    let mut y = usize_from_cmd();

    Level::new_filled_with(Some(Tile::Wall), x, y)
}

fn carve_dungeon(level: &mut Level) {
    println!("Not implemented.");
}

fn carve_maze(level: &mut Level) {
    print!("Input MazeGen startpos x coordinate: ");
    let mut x = usize_from_cmd();

    print!("Input MazeGen startpos y coordinate: ");
    let mut y = usize_from_cmd();

    level.apply(MazeGen::new(x,y).generate);
}

fn carve_rooms(level: &mut Level) {
    println!("Not implemented.");
}

fn png_export(level: &mut Level)

fn usize_from_cmd() -> usize {
    let mut num = String::new();

    io::stdin().read_line(&mut num)
        .expect("failed to read line");

    let num: usize = num.trim().parse()
        .expect("Please type a number!")
}
