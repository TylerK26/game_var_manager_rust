mod game_management;

use game_management::Game;

fn main() {
    let game = Game {
        title: String::from("Game"),
        id: 0,
        vars: vec!["1".to_string(), "2".to_string(), "3".to_string()]
    };
    println!("{}", game);
}
