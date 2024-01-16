mod game;
mod util;

/// Main function of the program.
///
/// This entry point for our program.
///
/// Prints out "Guess the number!" and runs the game using [game::run()]
fn main() {
    println!("Guess the number!");
    game::run();
}
