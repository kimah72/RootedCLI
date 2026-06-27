// Import the Serialize and Deserialize traits from the serde crate.
// These let the structs convert to and from JSON automatically.
use serde::{Serialize, Deserialize};
// Plant represents one plant record from the Rooted Upright DynamoDB table.
// #[derive(...)] auto-generates code so this struct can:
//   - Serialize: convert into JSON
//   - Deserialize: convert from JSON
//   - Debug: print the whole struct with {:?} for debugging
#[derive(Serialize, Deserialize, Debug)]
struct Plant {
    // The backend uses camelCase field names (e.g. "plantId"),
    // but Rust convention is snake_case (e.g. plant_id).
    // #[serde(rename = "...")] tells serde how to map between the two.
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
// impl blocks hold the behavior (methods) for a struct.
// The struct itself only holds data; impl is where we define what it can do.
impl Plant {
    // &self means this method borrows the Plant it's called on,
    // it only reads the plant's fields, it doesn't take ownership of them.
    // Returns a formatted String combining name and cultivar.
    fn display_name(&self) -> String {
        format!("{} ({})", self.name, self.cultivar)
    }
}
// CareLog represents one care event (watering, fertilizing, etc.)
// from the CareLogs DynamoDB table. Same serde pattern as Plant.
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
// Fetches all plants belonging to a given userId from the live AWS backend.
// async because it performs network I/O, which takes time and shouldn't
// block the rest of the program while waiting.
//
// Returns a Result: either Ok(Vec<Plant>) on success, or an Err with
// whatever reqwest::Error explains what went wrong (bad URL, network failure, etc).
async fn get_plants(user_id: &str) -> Result<Vec<Plant>, reqwest::Error> {

    // Build the full request URL, inserting the userId into the query string.
    let url = format!("https://xt71zwxu10.execute-api.us-east-1.amazonaws.com/plants?userId={}", user_id);

    // Send the GET request and wait (.await) for the response.
    // The ? operator means: if this fails, stop here and return the error immediately.
    let response = reqwest::get(&url).await?;

    // Parse the response body as JSON and deserialize it into a Vec<Plant>.
    // serde matches each JSON object's fields to a Plant struct using the
    // #[serde(rename = ...)] mappings defined above.
    let plants = response.json::<Vec<Plant>>().await?;

    // Wrap the result in Ok to match this function's promised return type.
    Ok(plants)
}
// #[tokio::main] sets up the async runtime that allows main() to use .await.
// Without it, Rust's plain fn main() can't run asynchronous code.
#[tokio::main]
async fn main() {
    // Call get_plants with a real Cognito user sub, and wait for the result.
    let result = get_plants("1418a448-d051-7053-ecea-c389e47c445c").await;

    // match checks whether the call succeeded (Ok) or failed (Err),
    // and runs different code depending on which happened.
    match result {
        Ok(plants) => {
            // Loop over every plant in the Vec. The & borrows the Vec,
            // so we can still use `plants` after the loop if needed,
            // rather than the loop taking ownership of it.
            for plant in &plants {
                // Conditional: if watch_for is empty, show a friendly placeholder
                // instead of printing nothing after the dash.
                if plant.watch_for.is_empty() {
                println!("{} - watch for: (none yet)", plant.display_name());
                } else {
                    println!("{} - watch for: {}", plant.display_name(), plant.watch_for);
                }
            }
        }
        Err(e) => {
            // If the API call failed, print the error instead of crashing.
            println!("Error fetching plants: {}", e);
        }
    }
}
