#![allow(dead_code)]

mod prelude {
    pub mod flags;
    pub mod notifs;
    pub mod engines;
    pub mod configs;
    pub mod count;
    pub mod context;
    pub mod loader;
    pub mod preprocessor;
}

use crate::prelude::loader;

fn main() {
    dotenvy::dotenv()
        .expect("Failed to load .env file.");
    let api = std::env::var("api")
        .expect("No api in env.");
    println!("{:?}", api);

    match loader::load_parser_config() {
        Ok(success) => {
            println!("Loaded config with notifs {:?}", success.notifs);
        },
        Err(e) => {
            eprintln!("Disaster occured: {:?}", e.0);
        }
    }
}
