mod game_management;

use std::path::Path;
use game_management::Game;
use game_management::get_game_name;
use game_management::get_game_image_url;
use game_management::get_game_ids;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Pretty much everything here is being used for testing right now
    // I don't know too much about rust so expect a lot of learning in writing this program
    let game = Game {
        title: String::from("Game"),
        id: 0,
        vars: vec!["1".to_string(), "2".to_string(), "3".to_string()]
    };
    let compat_data_path = Path::new("/home/tylerk/.steam/steam/steamapps/compatdata/");
    let name_test = get_game_name(620980).await?;
    let ids = get_game_ids(compat_data_path).await?;
    let image_url = get_game_image_url(620980);
    for item in ids.iter() {
        println!("{}", item);
    }
    Ok(())
}
