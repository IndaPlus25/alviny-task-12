use core::panic;
use std::{env::args, fmt, fs};
use ggez::{Context, ContextBuilder, GameResult, conf, event, mint::Point2};

const SCREEN_SIZE: (f32, f32) = (1920f32, 1080f32);
const CELL_SIZE: f32 = 50f32;

#[derive(fmt::Debug)]
enum CellState {Wall, Hallway}

struct AppState {
    player_position: Point2<f32>,
    player_direction: f32, // Player direction in degrees. 0 means straight right.
    map: Vec<Vec<CellState>>
}
impl AppState {
    fn new(_context: &mut Context, map: Vec<Vec<CellState>>) -> Option<AppState> {
        Some(AppState{
            player_position: Point2::from([0f32, 0f32]),
            player_direction: 0f32,
            map,
        })
    }
}

impl event::EventHandler for AppState {
    fn update(&mut self, context: &mut Context) -> std::result::Result<(), ggez::GameError> {
        todo!()
    }
    fn draw(&mut self, context: &mut Context) -> std::result::Result<(), ggez::GameError> {
        todo!()
    }
}




pub fn main() -> GameResult {

    let context_builder = ContextBuilder::new("raycaster", "alvinino")
        .window_setup(
            conf::WindowSetup::default()
                .title("NØllan Purgatory") // Set window title "Schack"
                //.icon("/icon.png"), // Set application icon
        )
        .window_mode(
            conf::WindowMode::default()
                .dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1) // Set window dimensions
                .resizable(false), // Fix window size
        );
    let (mut contex, event_loop) = context_builder.build().expect("Failed to build context.");

    let args: Vec<String> = args().collect();

    //let map: Vec<Vec<char>> = Vec::new();

    let map_path: String = if let Some(arg1) = args.get(1) {
        arg1.to_string()
    } else {
        "./resources/default_map.lvl".to_string()
    };

    if map_path.split(".").collect::<Vec<&str>>().last() != Some(&"lvl") {
        panic!("Error: Bad file ending. Expected 'lvl', got {}", map_path.split(".").collect::<Vec<&str>>().last().unwrap())
    } 
    let map_read_error_message = "Error: Unable to read map from file: ".to_owned() + &map_path;

    let map_string = fs::read_to_string(map_path).expect(&map_read_error_message);

    let map_chars = map_string
        .split("\n")
        .map(|element| 
            element
            .chars()
            .collect()
        )
        .collect::<Vec<Vec<char>>>();
    // end block
    
    let map: Vec<Vec<CellState>> = map_chars.into_iter().map(|row| {
        row.iter().map(|cell| {
            match cell {
                '.' => CellState::Hallway,
                '#' => CellState::Wall,
                _ => panic!("Invalid cell: {}", cell)
            }
        }).collect()
    }).collect();



    println!("{:?}", map);
    let state = AppState::new(&mut contex, map).expect("Failed to create state.");
    event::run(contex, event_loop, state) // Run window event loop
}

