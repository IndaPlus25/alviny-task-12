use core::panic;
use std::{env::args, fs};
use ggez::{Context, ContextBuilder, GameResult, conf, event, mint::Point2};

const DEFAULT_CELL_SIZE: u32 = 100;

#[derive(Debug)]
enum CellState {Wall, Hallway}

#[derive(Debug)]
struct CellMap {
    cells: Vec<Vec<CellState>>,
    width: u32,
    height: u32,
}
impl CellMap {
    fn from_2d_char_vec(map_chars: Vec<Vec<char>>) -> CellMap {

        let map: Vec<Vec<CellState>> = map_chars.into_iter().map(|row| {
            row.iter().map(|cell| {
                match cell {
                    '.' => CellState::Hallway,
                    '#' => CellState::Wall,
                    _ => panic!("Invalid cell: {}", cell)
                }
            }).collect()
        }).collect();

        let mut width: u32 = 0;
        let height: u32 = map.len().try_into().unwrap();
        for row in &map {
            if row.len() > width.try_into().unwrap() {
                width = row.len().try_into().unwrap();
            }
        }
        CellMap {
            cells: map, width, height
        }
    }
}

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

    

    let args: Vec<String> = args().collect();

    //let map: Vec<Vec<char>> = Vec::new();

    let map_path: String = if let Some(arg1) = args.get(1) {
        arg1.to_string()
    } else {
        "resources/default_map.lvl".to_string()
    };

    if map_path.split(".").collect::<Vec<&str>>().last() != Some(&"lvl") {
        panic!("Error: Bad file ending. Expected 'lvl', got {}", map_path.split(".").collect::<Vec<&str>>().last().unwrap())
    } 
    let map_read_error_message = "Error: Unable to read map from file: ".to_owned() + &map_path + "\nSystem panic";

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
    
    let map = CellMap::from_2d_char_vec(map_chars);


    println!("{:?}", map);

    let cell_size: u32 = if let Some(cell_size_argument_index) = args
        .iter()
        .position (|x| 
            (x.eq_ignore_ascii_case("-cs") || x.eq_ignore_ascii_case("--cellsize"))) 
    {
        args[cell_size_argument_index+1].parse::<u32>().expect("Could not convert cell size argument")
    } else {
        DEFAULT_CELL_SIZE
    };

    let context_builder = ContextBuilder::new("raycaster", "alvinino")
        .window_setup(
            conf::WindowSetup::default()
                .title("NØllan Purgatory") // Set window title
                //.icon("/icon.png"), // Set application icon
        )
        .window_mode(
            conf::WindowMode::default()
                .dimensions((cell_size * map.width) as f32, (cell_size * map.height) as f32) // Set window dimensions
                .resizable(false), // Fix window size
        );
    let (mut contex, event_loop) = context_builder.build().expect("Failed to build context.");


    let state = AppState::new(&mut contex, map.cells).expect("Failed to create state.");
    event::run(contex, event_loop, state) // Run window event loop
}

