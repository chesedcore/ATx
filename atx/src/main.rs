fn main() {
    dotenvy::dotenv()
        .expect("Failed to load .env file.");
    let api = std::env::var("api")
        .expect("No api in env.");
    println!("{}",api);
}
