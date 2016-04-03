extern crate image;
extern crate dungenon;

use std::io;
use dungenon::level::Level;
use dungenon::tile::Tile;
use dungenon::generator::MazeGen;

use image::png::PNGEncoder;
use image::RgbaImage;

fn main() {
    println!("Initializing level...");
    let mut level = init_level();

    println!("This is the dungenon-drawer. \nType Help for list of commands.");
    loop {
        println!("Enter command:");
        let mut command = String::new();
        io::stdin().read_line(&mut command)
        .expect("Failed to read line");
        println!("{}", command);
        match command.trim() {
            "dungeon" => carve_dungeon(&mut level),
            "maze" => carve_maze(&mut level),
            "room" => carve_rooms(&mut level),
            "reset" => level = init_level(),
            "export" => {
                png_export(&mut level);
            },
            "print" => print_level(&level),
            "help" => {
                println!("Help command is not implemented yet!");
            }
            "exit" => break,
            _ => println!("Invalid command"),
        }
    }
}

fn init_level() -> Level {
    println!("Input level width: ");
    let mut x = usize_from_cmd();


    println!("Input level height: ");
    let mut y = usize_from_cmd();

    Level::new_filled_with(Some(Tile::Wall), x, y)
}

fn carve_dungeon(level: &mut Level) {
    println!("Not implemented.");
}

fn carve_maze(level: &mut Level) {
    println!("Input MazeGen startpos x coordinate: ");
    let mut x = usize_from_cmd();

    println!("Input MazeGen startpos y coordinate: ");
    let mut y = usize_from_cmd();
    let mut mazegen = MazeGen::new(x,y);
    level.apply(|m| mazegen.generate(m));
}

fn carve_rooms(level: &mut Level) {
    println!("Not implemented.");
}

fn png_export(level: &mut Level) {
    let mut level_image = RgbaImage::new(level.get_height(), level.get_height());
    for y in 0 .. level.get_width() {
        for x in 0 .. level.get_height() {
            
        }
    }
}

fn print_level(level: &Level) {
    let mut string = String::new();
    for y in 0 .. level.get_width() {
        for x in 0 .. level.get_height() {
            match level[(x,y)] {
                Some(ref tile) => {
                    match tile {
                        &Tile::Wall => string.push('#'),
                        &Tile::Floor => string.push(' '),
                        &Tile::Void => string.push('*'),
                    }
                },
                None => string.push('*'),
            }
        }
        string.push('\n');
    }
    println!("{}", &string);
}

fn usize_from_cmd() -> usize {
    let mut num = String::new();

    io::stdin().read_line(&mut num)
        .expect("failed to read line");

    let num: usize = num.trim().parse()
        .expect("Please type a number!");
    num
}
