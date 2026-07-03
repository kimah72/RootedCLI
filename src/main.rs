// Serde traits for automatic JSON serialization and deserialization.
use serde::{Serialize, Deserialize};
// Engine trait required to call .decode() on the base64 engine.
use base64::Engine;
// io for terminal input; Write trait needed for flush().
use std::io::{self, Write};
// Datelike provides .month() on chrono date types.
use chrono::Datelike;
use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct Plant {
    // Backend uses camelCase; Rust convention is snake_case.
    // #[serde(rename)] maps between the two automatically.
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

// impl separates behavior from data. Plant holds the shape; impl holds what it can do.
impl Plant {
    // &self borrows the Plant without taking ownership, reads fields only.
    fn display_name(&self) -> String {
        format!("{} ({})", self.name, self.cultivar)
    }
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

impl CareLog {
    // Associated function (no &self) since it returns a seasonal constant,
    // not something specific to one CareLog instance.
    fn watering_threshold() -> i64 {
        let month = chrono::Local::now().month();
        match month {
            11 | 12 | 1 | 2 => 14, // Winter: every 14 days
            6 | 7 | 8 | 9 => 7,    // Summer: every 7 days
            _ => 10,                // Spring/Fall: every 10 days
        }
    }

    // Returns true if enough days have passed since this log entry
    // to warrant care again, based on the seasonal threshold.
    fn is_overdue(&self) -> bool {
        let threshold = CareLog::watering_threshold();
        // DynamoDB stores timestamps as ISO 8601 (e.g. "2026-07-01T03:01:42.629Z").
        // parse_from_rfc3339 handles this format; .map() extracts just the date portion.
        let logged = chrono::DateTime::parse_from_rfc3339(&self.date_logged)
            .map(|dt| dt.date_naive());
        match logged {
            Ok(date) => {
                let today = chrono::Local::now().date_naive();
                let days_since = (today - date).num_days();
                days_since >= threshold
            }
            // If the date can't be parsed, assume not overdue rather than crashing.
            Err(_) => false,
        }
    }
}

// Authenticates against Cognito using USER_PASSWORD_AUTH (direct credential flow,
// no browser redirect). Returns the user's sub, which scopes all API calls
// to their own data. Box<dyn Error> handles multiple possible error types.
async fn cognito_login(email: &str, password: &str) -> Result<String, Box<dyn std::error::Error>> {
    let body = serde_json::json!({
        "AuthFlow": "USER_PASSWORD_AUTH",
        "ClientId": "3isil38pk3rjglpvp0vse9q764", // identifies the app, not the user
        "AuthParameters": {
            "USERNAME": email,
            "PASSWORD": password
        }
    });

    let client = reqwest::Client::new();
    let response = client
        .post("https://cognito-idp.us-east-1.amazonaws.com/")
        .header("X-Amz-Target", "AWSCognitoIdentityProviderService.InitiateAuth")
        .header("Content-Type", "application/x-amz-json-1.1")
        .json(&body)
        .send()
        .await?;

    let json: serde_json::Value = response.json().await?;
    let id_token = json["AuthenticationResult"]["IdToken"]
        .as_str()
        .ok_or("No IdToken in response")?;

    // JWTs are three base64 sections separated by dots: header.payload.signature.
    // The payload (index 1) contains identity claims including the sub.
    let payload = id_token.split('.').nth(1).ok_or("Invalid token format")?;
    // JWTs use URL-safe base64 without padding.
    let decoded = base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(payload)?;
    let claims: serde_json::Value = serde_json::from_slice(&decoded)?;
    let sub = claims["sub"].as_str().ok_or("No sub in token")?.to_string();
    Ok(sub)
}

// Creates a new care log entry in DynamoDB. Generates a fresh UUID and
// today's date automatically so the caller only needs to supply the content.
async fn log_care(plant_id: &str, care_type: &str, notes: &str) -> Result<(), reqwest::Error> {
    let log_id = uuid::Uuid::new_v4().to_string();
    let date_logged = chrono::Local::now().format("%Y-%m-%d").to_string();

    // Field init shorthand: log_id and date_logged match their variable names.
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

// Fetches all plants for a given userId. Returns a Vec<Plant> deserialized
// from the JSON response. The ? operator propagates errors to the caller.
async fn get_plants(user_id: &str) -> Result<Vec<Plant>, reqwest::Error> {
    let url = format!("https://xt71zwxu10.execute-api.us-east-1.amazonaws.com/plants?userId={}", user_id);
    let response = reqwest::get(&url).await?;
    let plants = response.json::<Vec<Plant>>().await?;
    Ok(plants)
}

// Fetches all care logs for a given plantId.
// Used to check watering history and determine overdue status.
async fn get_care_logs(plant_id: &str) -> Result<Vec<CareLog>, reqwest::Error> {
    let url = format!("https://xt71zwxu10.execute-api.us-east-1.amazonaws.com/carelogs?plantId={}", plant_id);
    let response = reqwest::get(&url).await?;
    let logs = response.json::<Vec<CareLog>>().await?;
    Ok(logs)
}

// tokio::main drives the async runtime so main() can use .await.
#[tokio::main]
async fn main() {
    // Load credentials from .env file if present, otherwise fall back to terminal prompt.
    dotenv::dotenv().ok();

    let email = env::var("EMAIL").unwrap_or_else(|_| {
        print!("Email: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read email");
        input.trim().to_string()
    });

    let password = env::var("PASSWORD").unwrap_or_else(|_| {
        print!("Password: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read password");
        input.trim().to_string()
    });

    let sub = match cognito_login(&email, &password).await {  // <-- this stays
        Ok(s) => s,
        Err(e) => {
            println!("Login failed. Please check your email and password and try again.");
            println!("(Error details: {})", e);
            return;
        }
    };

    let result = get_plants(&sub).await;

    match result {
        Ok(plants) => {
            // Print plant list; conditional handles plants missing care instructions.
            for plant in &plants {
                if plant.care_instructions.is_empty() {
                    println!("{} - Care Instructions: (none yet)", plant.display_name());
                } else {
                    println!("{} - Care Instructions: {}", plant.display_name(), plant.care_instructions);
                }
            }

            // Numbered list for care log selection.
            // enumerate() pairs each plant with a 0-based index; +1 makes it 1-based for display.
            println!("\n--- Log Care ---");
            for (i, plant) in plants.iter().enumerate() {
                println!("{}.  {}", i + 1, plant.display_name());
            }

            // Optional logging: skip if the user just wants to check plant info.
            print!("\nWould you like to log care for a plant? (y/n): ");
            io::stdout().flush().unwrap();
            let mut log_choice = String::new();
            io::stdin().read_line(&mut log_choice).expect("Failed to read choice");

            if log_choice.trim().to_lowercase() == "y" {
                print!("\nSelect a plant number (or 0 to log for all plants): ");
                io::stdout().flush().unwrap();
                let mut selection = String::new();
                io::stdin().read_line(&mut selection).expect("Failed to read selection");
                // unwrap_or(0) handles non-numeric input gracefully.
                let selection: usize = selection.trim().parse().unwrap_or(0);

                print!("Care type (Watering, Fertilizing, Repotting, Pruning, Leaf Cleaning, Drama, Other): ");
                io::stdout().flush().unwrap();
                let mut care_type = String::new();
                io::stdin().read_line(&mut care_type).expect("Failed to read care type");
                let care_type = care_type.trim();

                print!("Notes (optional, press Enter to skip): ");
                io::stdout().flush().unwrap();
                let mut notes = String::new();
                io::stdin().read_line(&mut notes).expect("Failed to read notes");
                let notes = notes.trim();

                if selection == 0 {
                    // log-all: loop over every plant, handle failures individually
                    // so one failed write doesn't stop the rest from logging.
                    println!("Logging {} for all plants...", care_type);
                    for plant in &plants {
                        match log_care(&plant.plant_id, care_type, notes).await {
                            Ok(_) => println!("  ✓ {}", plant.display_name()),
                            Err(e) => println!("  ✗ {} (failed: {})", plant.display_name(), e),
                        }
                    }
                    println!("Done! All plants logged.");
                } else if selection >= 1 && selection <= plants.len() {
                    // Vec is zero-indexed; subtract 1 to match the user's 1-based selection.
                    let plant = &plants[selection - 1];
                    match log_care(&plant.plant_id, care_type, notes).await {
                        Ok(_) => println!("✓ Logged {} for {}.", care_type, plant.display_name()),
                        Err(e) => println!("✗ Failed to log {} for {}: {}", care_type, plant.display_name(), e),
                    }
                } else {
                    println!("Invalid selection.");
                }
            }
        }
        Err(e) => {
            println!("Error fetching plants: {}", e);
        }
    }

    // Check Andromeda's watering history and flag if overdue.
    // Filters to Watering logs only since other care types don't reset the watering clock.
    println!("\n--- Overdue Check ---");
    match get_care_logs("3aebd792-3ec8-42dc-a985-59d75cb2bdcd").await {
        Ok(logs) => {
            if logs.is_empty() {
                println!("No care logs found for Andromeda.");
            } else {
                // max_by on YYYY-MM-DD strings sorts correctly since the format is lexicographically ordered.
                let most_recent = logs.iter()
                    .filter(|log| log.care_type == "Watering")
                    .max_by(|a, b| a.date_logged.cmp(&b.date_logged));
                match most_recent {
                    Some(log) => {
                        let overdue = log.is_overdue();
                        println!("Most recent care: {} on {}", log.care_type, log.date_logged);
                        println!("Watering threshold: {} days", CareLog::watering_threshold());
                        if overdue {
                            println!("⚠ Andromeda is OVERDUE for care!");
                        } else {
                            println!("✓ Andromeda is on track.");
                        }
                    }
                    None => println!("No watering logs found for Andromeda."),
                }
            }
        }
        Err(e) => println!("Failed to fetch care logs: {}", e),
    }
}