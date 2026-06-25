struct Plant {
    plant_id: String,
    user_id: String,
    name: String,
    species: String,
    cultivar: String,
    lore: String,
    care_instructions: String,
    watch_for: String,
    date_added: String,
    image_url: String,
}

struct CareLog {
    log_id: String,
    plant_id: String,
    care_type: String,
    notes: String,
    date_logged: String,
}

fn main() {
    let my_plant = Plant {
        plant_id: String::from("001"),
        user_id: String::from("kimah"),
        name: String::from("Andromeda"),
        species: String::from("Monstera"),
        cultivar: String::from("Thai Constellation"),
        lore: String::from("A Walmart find that was actual treasure!"),
        care_instructions: String::from("Water when dry 2-3 inches below soil"),
        watch_for: String::from("pests"),
        date_added: String::from("2026-3-14"),
        image_url: String::from(""),
    };
    let care_log = CareLog {
        log_id: String::from("001"),
        plant_id: String::from("001"),
        care_type: String::from("Watering"),
        notes: String::from("Bottom watering"),
        date_logged: String::from("2026/06/24"),
    };
    println!("=== {} ===", my_plant.name);
    println!("Species: {}", my_plant.species);
    println!("Cultivar: {}", my_plant.cultivar);
    println!("Watch for: {}", my_plant.watch_for);

    println!("=== {} ===", care_log.care_type);
    println!("Notes: {}", care_log.notes);
    println!("Date Logged {}", care_log.date_logged);

    println!("Hello, world!");
}
