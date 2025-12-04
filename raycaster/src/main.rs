use core::panic;
use std::{env::args, fs};
use ggez::{Context, ContextBuilder, GameResult, conf, event, graphics::{self, Canvas, Color, DrawParam, Rect}, mint::Point2};

const DEFAULT_CELL_SIZE: u32 = 100;

const BLACK: Color = Color::new(0f32, 0f32, 0f32, 1f32);

const GRAY: Color = Color::new(128.0/255.0, 128.0/255.0, 128.0/255.0, 1f32);

const WHITE: Color = Color::new(1f32, 1f32, 1f32, 1f32);

const RED: Color = Color::new(1f32, 0f32, 0f32, 1f32);
const BLUE: Color = Color::new(0f32, 0f32, 1f32, 1f32);

#[derive(Debug, PartialEq)]
enum CellState {Wall, Hallway}
impl CellState {
    fn draw(&self,  canvas: &mut Canvas, ctx: &mut Context, coords: Point2<u32>, cell_size: u32) -> GameResult {
        let color: Color = match &self {
            CellState::Wall => GRAY,
            CellState::Hallway => WHITE,
        };

        let rect = Rect::new(
            (coords.x * cell_size) as f32, 
            (coords.y * cell_size) as f32, 
            cell_size as f32, 
            cell_size as f32
        );
        let rect_mesh = graphics::Mesh::new_rectangle(
            ctx, 
            graphics::DrawMode::fill(), 
            rect,
            color
        )?;

        let border_width = cell_size/20;
        let border_color = BLACK;

        let outline_mesh_top = graphics::Mesh::new_rectangle(
            ctx, 
            graphics::DrawMode::fill(), 
            Rect::new(
            (coords.x * cell_size) as f32, 
            (coords.y * cell_size) as f32, 
            cell_size as f32, 
            border_width as f32,
            ),
            border_color
        )?;
        let outline_mesh_right = graphics::Mesh::new_rectangle(
            ctx, 
            graphics::DrawMode::fill(), 
            Rect::new(
            ((coords.x + 1) * cell_size - border_width) as f32, 
            (coords.y * cell_size) as f32, 
            border_width as f32, 
            cell_size as f32,
            ),
            border_color
        )?;
        let outline_mesh_bottom = graphics::Mesh::new_rectangle(
            ctx, 
            graphics::DrawMode::fill(), 
            Rect::new(
            (coords.x * cell_size) as f32, 
            ((coords.y + 1) * cell_size - border_width) as f32, 
            cell_size as f32, 
            border_width as f32,
            ),
            border_color
        )?;
        let outline_mesh_left = graphics::Mesh::new_rectangle(
            ctx, 
            graphics::DrawMode::fill(), 
            Rect::new(
            (coords.x * cell_size) as f32, 
            (coords.y * cell_size) as f32, 
            border_width as f32, 
            cell_size as f32,
            ),
            border_color
        )?;
        canvas.draw(&rect_mesh, DrawParam::default());
        if *self == CellState::Wall {
            canvas.draw(&outline_mesh_top, DrawParam::default());
            canvas.draw(&outline_mesh_right, DrawParam::default());
            canvas.draw(&outline_mesh_bottom, DrawParam::default());
            canvas.draw(&outline_mesh_left, DrawParam::default());
        }
        
        Ok(())
    }
}

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
    map: CellMap,
    cell_size: u32, //cell size in px

}
impl AppState {
    fn new(_context: &mut Context, map: CellMap, cell_size: u32) -> Option<AppState> {
        Some(AppState{
            player_position: Point2::from([(map.width * cell_size/2) as f32, (map.height * cell_size / 2) as f32]),
            player_direction: 0f32,
            map,
            cell_size,
        })
    }
}

impl event::EventHandler for AppState {
    fn update(&mut self, context: &mut Context) -> std::result::Result<(), ggez::GameError> {

        Ok(())
    }
    fn draw(&mut self, context: &mut Context) -> std::result::Result<(), ggez::GameError> {
        let mut canvas: Canvas = graphics::Canvas::from_frame(
            context, 
            WHITE,
        );

        // draw map
        for (row_index, row) in self.map.cells.iter().enumerate() {
            for (cell_index, cell) in row.iter().enumerate() {
                cell.draw(&mut canvas, context, Point2::from([cell_index as u32, row_index as u32]), self.cell_size)?;
            }
        }

        // draw player sprite
        let player_sprite = graphics::Mesh::new_circle(
            context, 
            graphics::DrawMode::fill(),
            self.player_position, 
            (self.cell_size / 5) as f32, 
            0.67, 
            RED
        )?;
        canvas.draw(&player_sprite, graphics::DrawParam::default());

        canvas.finish(context)
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
    let split_pattern = 
    if map_string.contains("\r\n") {
        "\r\n"
    } else if map_string.contains("\n") {
        "\n"
    } else if map_string.contains("\r") {
        "\r"
    } else {
        "67" // gibberish string so it doesn't split at all
    };

    let map_chars = map_string
        .split(split_pattern)
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


    let state = AppState::new(&mut contex, map, cell_size).expect("Failed to create state.");
    event::run(contex, event_loop, state) // Run window event loop
}

