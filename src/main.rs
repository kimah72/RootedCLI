 use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug)]
struct Plant {
    #[serde(rename = "plantId")]
    plant_id: String,
    #[serde(rename = "userId")]
    user_id: String,
    name: String,
    species: String,
    cultivar: String,
    lore: String,
    #[serde(rename = "careInstructions")]
    care_instructions: String,
    #[serde(rename = "watchFor")]
    watch_for: String,
    #[serde(rename = "dateAdded")]
    date_added: String,
    #[serde(rename = "imageUrl")]
    image_url: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct CareLog {
    #[serde(rename = "logId")]
    log_id: String,
    #[serde(rename = "plantId")]
    plant_id: String,
    #[serde(rename = "careType")]
    care_type: String,
    notes: String,
    #[serde(rename = "dateLogged")]
    date_logged: String,
}
async fn get_plants(user_id: &str) -> Result<Vec<Plant>, reqwest::Error> {
    let url = format!("https://xt71zwxu10.execute-api.us-east-1.amazonaws.com/plants?userId={}", user_id);
    let response = reqwest::get(&url).await?;
    let plants = response.json::<Vec<Plant>>().await?;
    Ok(plants)
}
  #[tokio::main]
  async fn main() {
    let result = get_plants("1418a448-d051-7053-ecea-c389e47c445c").await;

    match result {
        Ok(plants) => {
            println!("Got {} plants!", plants.len());
        }
        Err(e) => {
            println!("Error fetching plants: {}", e);
        }
    }
}
