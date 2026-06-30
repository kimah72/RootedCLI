// Import the Serialize and Deserialize traits from the serde crate.
// These let the structs convert to and from JSON automatically.
use serde::{Serialize, Deserialize};
use base64::Engine;
use std::io::{self, Write};
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
async fn cognito_login(email: &str, password: &str) -> Result<String, Box<dyn std::error::Error>> {
    // build the request body
    let body = serde_json::json!({
        "AuthFlow": "USER_PASSWORD_AUTH",
        "ClientId": "3isil38pk3rjglpvp0vse9q764",
        "AuthParameters": {
            "USERNAME": email,
            "PASSWORD": password
        }
    });
    // send the POST request to Cognito
    let client = reqwest::Client::new();
    let response = client
        .post("https://cognito-idp.us-east-1.amazonaws.com/")
        .header("X-Amz-Target", "AWSCognitoIdentityProviderService.InitiateAuth")
        .header("Content-Type", "application/x-amz-json-1.1")
        .json(&body)
        .send()
        .await?;
    // parse the response and extract the ID token
    let json: serde_json::Value = response.json().await?;
    let id_token = json["AuthenticationResult"]["IdToken"]
        .as_str()
        .ok_or("No IdToken in response")?;
    // decode the ID token and extract the sub
    let payload = id_token.split('.').nth(1).ok_or("Invalid token format")?;
    let decoded = base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(payload)?;
    let claims: serde_json::Value = serde_json::from_slice(&decoded)?;
    let sub = claims["sub"].as_str().ok_or("No sub in token")?.to_string();
    Ok(sub)
}

async fn log_care(plant_id: &str, care_type: &str, notes: &str) -> Result<(), reqwest::Error> {
    let log_id = uuid::Uuid::new_v4().to_string();
    let date_logged = chrono::Local::now().format("%Y-%m-%d").to_string();

    let care_log = CareLog {
        log_id,
        plant_id: plant_id.to_string(),
        care_type: care_type.to_string(),
        notes: notes.to_string(),
        date_logged,
    };
    let client = reqwest::Client::new();
    let response = client
        .post("https://xt71zwxu10.execute-api.us-east-1.amazonaws.com/carelogs")
        .json(&care_log)
        .send()
        .await?;
    if response.status().is_success() {
    println!("Care log added successfully!");
    } else {
        println!("Failed to add care log: {}", response.status());
    }

    Ok(())
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
    print!("Email: ");
    io::stdout().flush().unwrap();
    let mut email = String::new();
    io::stdin().read_line(&mut email).expect("Failed to read email");
    let email = email.trim();

    print!("Password: ");
    io::stdout().flush().unwrap();
    let mut password = String::new();
    io::stdin().read_line(&mut password).expect("Failed to read password");
    let password = password.trim();

    let sub = match cognito_login(email, password).await {
        Ok(s) => s,
        Err(e) => {
            println!("Login failed. Please check your email and password and try again.");
            println!("(Error details: {})", e);
            return;
        }
    };

    let result = get_plants(&sub).await;

    // match checks whether the call succeeded (Ok) or failed (Err),
    // and runs different code depending on which happened.
    match result {
        Ok(plants) => {
            // Print plant list with care instructions
            for plant in &plants {
                if plant.care_instructions.is_empty() {
                    println!("{} - Care Instructions: (none yet)", plant.display_name());
                } else {
                    println!("{} - Care Instructions: {}", plant.display_name(), plant.care_instructions);
                }
            }

            // Print numbered list for selection
            println!("\n--- Log Care ---");
            for (i, plant) in plants.iter().enumerate() {
                println!("{}.  {}", i + 1, plant.display_name());
            }

            // Prompt user to select a plant by number
            print!("\nSelect a plant number (or 0 to log for all plants): ");
            io::stdout().flush().unwrap();
            let mut selection = String::new();
            io::stdin().read_line(&mut selection).expect("Failed to read selection");
            let selection: usize = selection.trim().parse().unwrap_or(0);

            // Prompt for care type
            print!("Care type (Watering, Fertilizing, Repotting, Pruning, Leaf Cleaning, Drama, Other): ");
            io::stdout().flush().unwrap();
            let mut care_type = String::new();
            io::stdin().read_line(&mut care_type).expect("Failed to read care type");
            let care_type = care_type.trim();

            // Prompt for optional notes
            print!("Notes (optional, press Enter to skip): ");
            io::stdout().flush().unwrap();
            let mut notes = String::new();
            io::stdin().read_line(&mut notes).expect("Failed to read notes");
            let notes = notes.trim();

            // If 0, log care for all plants. Otherwise log for the selected plant.
            if selection == 0 {
                println!("Logging {} for all plants...", care_type);
                for plant in &plants {
                    match log_care(&plant.plant_id, care_type, notes).await {
                        Ok(_) => println!("  ✓ {}", plant.display_name()),
                        Err(e) => println!("  ✗ {} (failed: {})", plant.display_name(), e),
                    }
                }
                println!("Done! All plants logged.");
            } else if selection >= 1 && selection <= plants.len() {
                // Vec is zero-indexed, so subtract 1 to get the right plant
                let plant = &plants[selection - 1];
                match log_care(&plant.plant_id, care_type, notes).await {
                    Ok(_) => println!("✓ Logged {} for {}.", care_type, plant.display_name()),
                    Err(e) => println!("✗ Failed to log {} for {}: {}", care_type, plant.display_name(), e),
                }
            } else {
                println!("Invalid selection.");
            }
        }
        Err(e) => {
            // If the API call failed, print the error instead of crashing.
            println!("Error fetching plants: {}", e);
        }
    }
}
