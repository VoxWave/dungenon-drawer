extern crate image;
extern crate dungenon;
extern crate cast;

use std::io;
use std::path::PathBuf;

use dungenon::level::Level;
use dungenon::tile::Tile;
use dungenon::generator::{MazeGen, RoomGen, DungeonGen};
use dungenon::util::Error;

use image::RgbImage;
use image::Rgb;

use std::collections::HashMap;

fn main() {
    Drawer::new().start();
}

struct Drawer {
    pub colors: HashMap<Tile, Rgb<u8>>,
}

impl Drawer {
    pub fn new() -> Drawer {
        let mut colors = HashMap::new();
        colors.insert(Tile::Wall(0), Rgb([127u8, 127u8, 127u8]));
        colors.insert(Tile::Floor(0), Rgb([200u8, 200u8, 200u8]));
        colors.insert(Tile::Void(0), Rgb([0u8, 0u8, 0u8]));
        Drawer{
            colors: colors,
        }
    }

    pub fn start(&mut self) {
        println!("Initializing level...");
        let mut level = self.init_level();

        println!("This is the dungenon-drawer. \nType Help for list of commands.");
        loop {
            println!("Enter command:");
            let command = Self::string_from_cmd();
            match command.trim() {
                "dungeon" => Self::carve_dungeon(&mut level),
                "maze" => Self::carve_maze(&mut level),
                "room" => Self::carve_rooms(&mut level),
                "reset" => level = self.init_level(),
                "colors" => self.change_colors(),
                "export" => {
                    self.png_export(&mut level);
                },
                "print" => Self::print_level(&level),
                "help" => {
                    println!("Help command is not implemented yet!");
                }
                "exit" => break,
                _ => println!("Invalid command."),
            }
        }
    }

    fn change_colors(&mut self) {
        use cast;
        use std::u8;
        loop {
            println!("Which color would you like to change? (floor, wall, void), Type exit if you're done with modifying colors.");
            let command = Self::string_from_cmd();
            let key = match command.trim() {
                "floor" => Tile::Floor(0),
                "wall" => Tile::Wall(0),
                "void" => Tile::Void(0),
                "exit" => break,
                _ => {
                    println!("Invalid command.");
                    continue;
                },
            };
            println!("Input red value for the color:");
            let r = cast::u8(Self::usize_from_cmd()).unwrap_or(u8::MAX);

            println!("Input green value for the color:");
            let g = cast::u8(Self::usize_from_cmd()).unwrap_or(u8::MAX);

            println!("Input blue value for the color:");
            let b = cast::u8(Self::usize_from_cmd()).unwrap_or(u8::MAX);
            self.colors.insert(key, Rgb([r,g,b]));
        }
    }

    fn init_level(&self) -> Level<Tile> {
        println!("Input level width: ");
        let x = Self::usize_from_cmd();


        println!("Input level height: ");
        let y = Self::usize_from_cmd();

        Level::new_filled_with(Tile::Wall(0), x, y)
    }

    fn carve_dungeon(level: &mut Level<Tile>) {
        println!("Creating DungeonGen...");
        let mazegen = Self::create_mazegen();
        let roomgen = Self::create_roomgen();
        let mut dungeongen = DungeonGen::new(mazegen, roomgen);
        level.apply(|m| dungeongen.generate(m));
    }

    fn create_mazegen() -> MazeGen {
        println!("Input MazeGen startpos x coordinate: ");
        let x = Self::usize_from_cmd();

        println!("Input MazeGen startpos y coordinate: ");
        let y = Self::usize_from_cmd();
        MazeGen::new(x,y)
    }

    fn carve_maze(level: &mut Level<Tile>) {
        let mut mazegen = Self::create_mazegen();
        level.apply(|m| mazegen.generate(m));
    }

    fn create_roomgen() -> RoomGen {
        println!("Input min room size:");
        let min_room_size = Self::usize_from_cmd();

        println!("Input max room size:");
        let max_room_size = Self::usize_from_cmd();

        println!("Input min room distance:");
        let room_distance = Self::usize_from_cmd();

        println!("Input room placement amount (Something high preferably):");
        let attempts = Self::u64_from_cmd();

        RoomGen::new(min_room_size, max_room_size, room_distance, attempts)
    }

    fn carve_rooms(level: &mut Level<Tile>) {
        let mut roomgen = Self::create_roomgen();
        level.apply(|m| roomgen.generate(m));
    }

    fn png_export(&self, level: &mut Level<Tile>) {
        let mut level_image = RgbImage::new(level.get_width() as u32, level.get_height() as u32);
        for x in 0 .. level.get_width() {
            for y in 0 .. level.get_height() {
                level_image.put_pixel(x as u32, y as u32, self.tile_to_color(&level.get_tile_with_tuple((x,y))));
            }
        }
        let mut p = PathBuf::new();
        println!("Enter png name:");
        p.push(Self::string_from_cmd().trim());
        p.set_extension("png");
        level_image.save(p.as_path()).expect("Something went wrong when saving the png.");
    }

    fn tile_to_color(&self, tile: &Result<&Tile, Error>) -> Rgb<u8> {
        match *tile {
            Ok(ref tile) => {
                    match self.colors.get(tile) {
                        Some(color) => color.clone(),
                        None => Rgb([255u8, 0u8, 255u8]),
                    }
            },
            Err(Error::IndexOutOfBounds) => panic!("Tile png import failed. Level indexing went out of bounds."),
        }
    }


    fn print_level(level: &Level<Tile>) {
        use dungenon::util::Error;
        let mut string = String::new();
        for y in 0 .. level.get_width() {
            for x in 0 .. level.get_height() {
                match level.get_tile_with_tuple((x,y)) {
                    Ok(tile) => {
                        match tile {
                            &Tile::Wall(_) => string.push('#'),
                            &Tile::Floor(_) => string.push(' '),
                            &Tile::Void(_) => string.push('*'),
                        }
                    },
                    Err(Error::IndexOutOfBounds) => panic!("IndexOutOfBounds occurred. Level printing wasn't implemented properly."),
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

    fn u64_from_cmd() -> u64 {
        let mut num = String::new();

        io::stdin().read_line(&mut num)
            .expect("failed to read line");

        let num: u64 = num.trim().parse()
            .expect("Please type a number!");
        num
    }

    fn string_from_cmd() -> String {
        let mut string = String::new();
        io::stdin().read_line(&mut string)
        .expect("Failed to read line");
        string
    }
}
