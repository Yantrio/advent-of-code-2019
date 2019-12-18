use computer::{Channel, Computer, IOMode};
use crossbeam_channel::{unbounded, Receiver};

use std::cmp::Ordering;
use std::convert::TryFrom;
use std::fs::read_to_string;
use std::thread;

use num_enum::TryFromPrimitive;

use ggez::conf::WindowMode;
use ggez::event;
use ggez::graphics::{self, Color, DrawMode, DrawParam, FillOptions, Mesh, MeshBuilder, Rect};
use ggez::{Context, ContextBuilder, GameResult};

#[derive(Debug, TryFromPrimitive, Clone, Copy, PartialEq, Eq)]
#[repr(i64)]
enum TileType {
    Empty = 0,
    Wall = 1,
    Block = 2,
    HoriztonalPaddle = 3,
    Ball = 4,
    Score = 99,
}

impl TileType {
    fn from_i64(input: i64) -> TileType {
        match TileType::try_from(input) {
            Ok(dir) => dir,
            Err(e) => panic!("Cannot get tile type from input: {}", e),
        }
    }

    fn to_color(&self) -> Color {
        match self {
            TileType::Empty => Color::new(0.0, 0.0, 0.0, 0.0),
            TileType::Wall => Color::new(0.5, 0.5, 0.5, 1.0),
            TileType::Block => Color::new(0.0, 0.0, 0.0, 1.0),
            TileType::HoriztonalPaddle => Color::new(0.0, 1.0, 0.0, 1.0),
            TileType::Ball => Color::new(0.0, 0.0, 1.0, 1.0),
            TileType::Score => Color::new(0.0, 0.0, 0.0, 0.0),
        }
    }
}

#[derive(Debug, Hash, PartialEq, Copy, Clone)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Tile {
    position: Point,
    tile_type: TileType,
    score: i64,
}

impl Tile {
    fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        if self.tile_type == TileType::Empty {
            return Ok(());
        }
        let rect = graphics::Rect::new(
            self.position.x as f32 * TILE_SIZE,
            self.position.y as f32 * TILE_SIZE,
            TILE_SIZE - 2.0,
            TILE_SIZE - 2.0,
        );
        let r = Mesh::new_rectangle(ctx, DrawMode::fill(), rect, self.tile_type.to_color())?;
        graphics::draw(ctx, &r, DrawParam::default())
    }
}

struct MyGame {
    tiles: Vec<Tile>,
    input_chan: Channel,
    tile_reciever: Receiver<Tile>,
    score: i64,
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        let input = read_to_string("input").expect("failed to read input file");
        let mut c = Computer::from_string(&input, IOMode::Channel);
        c.mem.memory[0] = 2;
        c.enable_logger = false;

        let output_chan = c.output_channel.clone();
        let input_chan = c.input_channel.clone();
        let (tile_sender, tile_reciever) = unbounded();

        thread::spawn(move || {
            &c.run();
        });
        thread::spawn(move || loop {
            let mut new_tiles = Vec::new();
            update_from_output(&output_chan, &mut new_tiles);
            for t in new_tiles {
                tile_sender.send(t).unwrap();
            }
        });

        let g = MyGame {
            tile_reciever,
            input_chan,
            tiles: Vec::new(),
            score: 0,
        };

        g
    }

    pub fn get_tiles_at_pos(tiles: &mut std::vec::Vec<Tile>, p: Point) -> Option<&mut Tile> {
        tiles.iter_mut().find(|tile| tile.position == p)
    }
    pub fn get_tile_by_type(tiles: &Vec<Tile>, t: TileType) -> Option<&Tile> {
        tiles.iter().find(|&tile| tile.tile_type == t)
    }

    pub fn set_tile_at_pos(&mut self, p: Point, t: TileType) {
        let found = MyGame::get_tiles_at_pos(&mut self.tiles, p);
        match found {
            None => self.tiles.push(Tile {
                position: p,
                tile_type: t,
                score: 0,
            }),
            Some(found) => {
                found.tile_type = t;
            }
        }
    }

    fn follow_ball(&mut self) -> Option<()> {
        let paddle = MyGame::get_tile_by_type(&self.tiles, TileType::HoriztonalPaddle)?;
        let ball = MyGame::get_tile_by_type(&self.tiles, TileType::Ball)?;

        let x_diff = ball.position.x - paddle.position.x;
        match x_diff.cmp(&0) {
            Ordering::Greater => self.input_chan.send(1),
            Ordering::Equal => self.input_chan.send(0),
            Ordering::Less => self.input_chan.send(-1),
        }

        Some(())
    }

    fn update_from_computer(&mut self) {
        //now read it all
        while !self.tile_reciever.is_empty() {
            let new_tile = self.tile_reciever.recv().unwrap();
            if new_tile.tile_type == TileType::Score {
                self.score = new_tile.score;
                println!("Score Updated: {}", self.score);
                continue;
            }

            self.set_tile_at_pos(new_tile.position, new_tile.tile_type);
        }
    }
}

const TILE_SIZE: f32 = 16.0;

impl ggez::event::EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.update_from_computer();
        self.follow_ball();
        //wait for update
        while self.tile_reciever.is_empty() {}

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);

        build_rect(ctx);

        for t in self.tiles.iter().filter(|t| t.tile_type != TileType::Score) {
            t.draw(ctx).unwrap();
        }

        graphics::present(ctx)?;
        Ok(())
    }
}

fn build_rect(ctx: &mut Context) {
    MeshBuilder::new()
        .rectangle(
            DrawMode::Fill(FillOptions::DEFAULT),
            Rect::new(0.0, 0.0, 10.0, 10.0),
            graphics::BLACK,
        )
        .build(ctx)
        .unwrap();
}

fn main() {
    let input = read_to_string("input").expect("failed to read input file");

    part1(&input);
    part2();
}

fn part2() {
    let (mut ctx, mut event_loop) = ContextBuilder::new("hello_ggez", "awesome_person")
        .build()
        .unwrap();

    graphics::set_mode(
        &mut ctx,
        WindowMode {
            width: 600.0,
            height: 400.0,
            borderless: false,
            fullscreen_type: ggez::conf::FullscreenType::Windowed,
            maximized: false,
            resizable: false,
            min_width: 0.0,
            max_width: 0.0,
            min_height: 0.0,
            max_height: 0.0,
        },
    )
    .unwrap();

    graphics::set_screen_coordinates(&mut ctx, ggez::graphics::Rect::new(0.0, 0.0, 600.0, 400.0))
        .unwrap();

    let mut game = MyGame::new(&mut ctx);
    match event::run(&mut ctx, &mut event_loop, &mut game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}

fn part1(input: &str) {
    let mut c = Computer::from_string(&input, IOMode::Channel);

    let mut tiles = Vec::new();
    c.run();
    while !c.output_channel.receiver.is_empty() {
        update_from_output(&c.output_channel, &mut tiles);
    }

    println!(
        "Solution part 1:{}",
        tiles
            .iter()
            .filter(|t| t.tile_type == TileType::Block)
            .count()
    );
}

fn update_from_output(c: &Channel, tiles: &mut Vec<Tile>) {
    let position = Point {
        x: c.recv(),
        y: c.recv(),
    };

    tiles.push(match position.x == -1 && position.y == 0 {
        true => Tile {
            position,
            tile_type: TileType::Score,
            score: c.recv(),
        },
        false => Tile {
            position,
            score: 0,
            tile_type: TileType::from_i64(c.recv()),
        },
    });
}
