use std::fmt;
use serde_json::Value;

pub struct Game {
    pub title: String,
    pub id: u32,
    pub vars: Vec<String>
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.title)
    }
}
pub async fn get_game_name(game_id: u32) -> Result<String, Box<dyn std::error::Error>> {
    /* Gets the name of the game provided */
    let client = reqwest::Client::new();

    let json: Value = client.get(format!("https://store.steampowered.com/api/appdetails?appids={}", game_id))
        .send()
        .await?
        .json()
        .await?;

    Ok(json[game_id.to_string()]["data"]["name"]
        .as_str()
        .ok_or("Could not get game name, ID likely invalid")?
        .to_string())
}

pub fn get_game_image_url(game_id: u32) -> String {
    format!("https://steamcdn-a.akamaihd.net/steam/apps/{}/library_600x900_2x.jpg", game_id)
}