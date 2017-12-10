extern crate piston_window;
extern crate piston;
extern crate game_of_life_base;

use game_of_life_base::game_of_life_logic::*;
use piston_window::*;

fn main() {
    // Setup
    let mut goll = GameOfLifeLogic::new();
    for (i, arg) in std::env::args().enumerate() {
        if i == 0 { continue; }
        let mut split = arg.split(',');
        let x: i32 = split.next().unwrap().parse().unwrap();
        let y: i32 = split.next().unwrap().parse().unwrap();
        println!("Your input was: {} and {}!", x, y);
        goll.add(x, y);
    }

    let cell_size: f64 = 5.0;

    // Build window
    let mut window: PistonWindow =
        WindowSettings::new("Conway's Game of Life!", [640, 480])
            .exit_on_esc(true).build().unwrap();

    // Draw on screen
    while let Some(event) = window.next() {
        if let Some(button_args) = event.button_args() {
            match button_args.button {

            }
            goll.tick();
        }

        window.draw_2d(&event, |context, graphics| {
            clear([1.0; 4], graphics);

            for cell in &goll.cells {
                rectangle([0.27, 0.48, 0.24, 1.0],
                          [cell.x as f64 * cell_size, cell.y as f64 * cell_size, cell_size, cell_size],
                          context.transform, graphics)
            }
        });
    }
}
// Input: GameOfLife.exe 0,0 0,1 0,2 0,3 0,4 0,5 0,6

// Glider right bottom
// Input: GameOfLife.exe 2,6 3,6 4,6 4,5 3,4

// Glider: right top
// Input: GameOfLife.exe 3,6 4,6 5,6 5,7 4,8