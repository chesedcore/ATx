mod prelude {
    pub mod flags;
    pub mod notifs;
    pub mod engines;
    pub mod configs;
    pub mod count;
    pub mod context;
}

fn main() {
    dotenvy::dotenv()
        .expect("Failed to load .env file.");
    let api = std::env::var("api")
        .expect("No api in env.");
    println!("{:?}", api);
}
