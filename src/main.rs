mod game_management;

use game_management::Game;
use game_management::get_game_name;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Pretty much everything here is being used for testing right now
    // I don't know too much about rust so expect a lot of learning in writing this program
    let game = Game {
        title: String::from("Game"),
        id: 0,
        vars: vec!["1".to_string(), "2".to_string(), "3".to_string()]
    };
    let name_test = get_game_name(620980).await?;
    println!("{}", name_test);
    Ok(())
}
